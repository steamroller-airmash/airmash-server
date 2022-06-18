use std::sync::Arc;
use std::time::{Duration, Instant};

use airmash_protocol::{MobType, Vector2};
use fxhash::FxHashSet as HashSet;
use hecs::{Entity, EntityBuilder, NoSuchEntity};
use nalgebra::vector;
use smallvec::SmallVec;

use crate::component::*;
use crate::config::MissilePrototypeRef;
use crate::event::{EntitySpawn, MobSpawn, PlayerFire};
use crate::network::{ConnectionId, ConnectionMgr};
use crate::protocol::{v5, ServerPacket};
use crate::resource::collision::LayerSpec;
use crate::resource::{Config, LastFrame, ThisFrame};
use crate::AirmashGame;

/// Info required to spawn a new missile.
#[derive(Clone, Copy, Debug)]
pub struct FireMissileInfo {
  /// Starting offset of the missile, relative to the plane that is firing it.
  /// This will be rotated into the plane's frame of reference.
  pub pos_offset: Vector2<f32>,
  /// Direction that the missile will accelerate in, relative to the direction
  /// the plane is facing when it fires.
  pub rot_offset: f32,
  /// Type of the missile.
  pub ty: MobType,
}

impl AirmashGame {
  /// Get the time at which the current frame occurred. This should be preferred
  /// over using `Instant::now`.
  pub fn this_frame(&self) -> Instant {
    self.resources.read::<ThisFrame>().0
  }

  /// Get the time at which the last frame occurred.
  pub fn last_frame(&self) -> Instant {
    self.resources.read::<LastFrame>().0
  }

  /// Get the delta between this frame and the last.
  pub fn frame_delta(&self) -> f32 {
    crate::util::convert_time(self.this_frame() - self.last_frame())
  }

  /// Get the time at which the server started.
  pub fn start_time(&self) -> Instant {
    use crate::resource::StartTime;

    self.resources.read::<StartTime>().0
  }

  /// Get the entity corresponding to the provided id.
  ///
  /// TODO: Currently this performs a linear scan on all entities. It should be
  /// accelerated with a hashmap.
  pub fn find_entity_by_id(&self, id: u16) -> Option<Entity> {
    let mut query = self
      .world
      .query::<(Option<&IsPlayer>, Option<&IsMissile>, Option<&IsMob>)>();
    for (ent, (player, missile, mob)) in query.iter() {
      if player.is_none() && missile.is_none() && mob.is_none() {
        continue;
      }

      if ent.id() as u16 == id {
        return Some(ent);
      }
    }

    None
  }

  /// Fire a number of missiles from a plane.
  ///
  /// This will create the entities for the missiles and also dispatch the
  /// required events.
  pub fn fire_missiles(
    &mut self,
    player: Entity,
    missiles: &[FireMissileInfo],
  ) -> Result<SmallVec<[Entity; 3]>, hecs::NoSuchEntity> {
    let mut entities = SmallVec::new();
    let mut builders = SmallVec::<[EntityBuilder; 5]>::new();

    let config = self.resources.read::<Config>();
    let this_frame = self.resources.read::<ThisFrame>().0;

    let (pos, rot, vel, team, &upgrades, last_fire_time, _) = self
      .world
      .query_one_mut::<(
        &Position,
        &Rotation,
        &Velocity,
        &Team,
        &Upgrades,
        &mut LastFireTime,
        &IsPlayer,
      )>(player)
      .map_err(|_| NoSuchEntity)?;

    let speed = vel.norm();
    let upg_factor = config.upgrades.missile.factor[upgrades.missile as usize];

    for info in missiles {
      let rot = rot.0 + info.rot_offset;
      let pos = pos.0 - crate::util::rotate(info.pos_offset, rot);
      let missile = config.mobs[info.ty].missile.expect("Mob was not a missile");

      // Rotate starting angle 90 degrees so that it's inline with the plane. Change
      // this and missiles will shoot sideways
      let dir = vector![rot.sin(), -rot.cos()];
      let vel = dir * (missile.base_speed + speed * missile.speed_factor) * upg_factor;

      let mut builder = crate::defaults::build_default_missile();
      builder
        .add(Position(pos))
        .add(Velocity(vel))
        .add(Accel(dir * missile.accel))
        .add(info.ty)
        .add(Owner(player))
        .add(Team(team.0))
        .add(SpawnTime(this_frame))
        .add(MissileTrajectory {
          start: pos,
          maxdist: missile.distance,
        });

      builders.push(builder);
    }

    last_fire_time.0 = this_frame;

    drop(config);

    for mut builder in builders {
      let entity = self.world.spawn(builder.build());

      if entity.id() > u16::MAX as _ {
        warn!("Missile created with ID > 65535 that is too large. Dropping it");
        let _ = self.world.despawn(entity);
        continue;
      }

      self.dispatch(EntitySpawn { entity });

      entities.push(entity);
    }

    if !missiles.is_empty() {
      self.dispatch(PlayerFire {
        player,
        missiles: entities.clone(),
      });
    }

    Ok(entities)
  }

  /// Fire a number of missiles from a plane, automatically determining the
  /// offsets and angles for each fired missile. This method gives less control
  /// than [fire_missiles](self::AirmashGame::fire_missiles) but is easier to
  /// use and is generally what you want unless you need to control exactly
  /// where the missiles are being fired from.
  pub fn fire_missiles_count(
    &mut self,
    player: Entity,
    mut count: usize,
    missile: MissilePrototypeRef,
  ) -> Result<SmallVec<[Entity; 3]>, hecs::QueryOneError> {
    // Only fire an odd number of missiles
    if count % 2 == 0 {
      count += 1;
    }

    assert!(
      count <= 255,
      "tried to spawn {} missiles at once - at most 255 can be spawned at a time",
      count
    );

    let scale_offset = |count: f32, base: f32| 2.0 * base * (1.0 - 1.0 / (1.0 + count));

    let (&plane, side, _) = self
      .world
      .query_one_mut::<(&PlaneType, &mut MissileFiringSide, &IsPlayer)>(player)?;

    let halfcnt = (count / 2) as f32;
    let config = self.resources.read::<Config>();
    let pconfig = &config.planes[plane];

    let total_angle = halfcnt * pconfig.missile_inferno_angle;
    let total_offset_x = scale_offset(halfcnt, pconfig.missile_inferno_offset_x);
    let total_offset_y = scale_offset(halfcnt, pconfig.missile_inferno_offset_y);

    let side = std::mem::replace(side, side.reverse());
    let start_x = pconfig.missile_offset.x;
    let start_y = pconfig.missile_offset.y * side.multiplier();

    let mut infos = SmallVec::<[_; 3]>::new();
    infos.push(FireMissileInfo {
      pos_offset: vector![start_y, start_x],
      rot_offset: 0.0,
      ty: missile.server_type,
    });

    for i in 1..=(count / 2) {
      let frac = (i as f32) / halfcnt;
      let angle = total_angle * frac;

      infos.push(FireMissileInfo {
        pos_offset: vector![
          start_y + total_offset_y * frac,
          start_x - total_offset_x * frac
        ],
        rot_offset: -angle,
        ty: missile.server_type,
      });
      infos.push(FireMissileInfo {
        pos_offset: vector![
          start_y - total_offset_y * frac,
          start_x - total_offset_x * frac
        ],
        rot_offset: angle,
        ty: missile.server_type,
      });
    }

    drop(config);
    Ok(self.fire_missiles(player, &infos)?)
  }

  /// Spawn a mob (upgrade or powerup).
  ///
  /// # Panics
  /// Panics if `mob` is not one of `Inferno`, `Shield`, or `Upgrade`. You need
  /// to use [`fire_missiles`] instead if you want to create a missile-type
  /// entity.
  ///
  /// [`fire_missiles`]: crate::AirmashGame::fire_missiles
  pub fn spawn_mob(&mut self, mob: MobType, pos: Vector2<f32>, lifetime: Duration) -> Entity {
    assert!(
      matches!(mob, MobType::Inferno | MobType::Shield | MobType::Upgrade),
      "Can only spawn stationary mobs"
    );

    let this_frame = self.this_frame();
    let entity = self
      .world
      .spawn((mob, Position(pos), Expiry(this_frame + lifetime), IsMob));

    self.dispatch(EntitySpawn { entity });
    self.dispatch(MobSpawn { mob: entity });

    entity
  }

  /// Update the score of `player` by `delta`. This method takes care of
  /// updating all the dependent data and emitting the required event.
  ///
  /// # Errors
  /// Returns an error if the entity pointed to by `player` is not a player
  /// or if it doesn't have the right set of components.
  pub fn update_score(&mut self, player: Entity, delta: i32) -> Result<(), hecs::NoSuchEntity> {
    use crate::event::PlayerScoreUpdate;

    let (score, earnings, _) = self
      .world
      .query_one_mut::<(&mut Score, &mut Earnings, &IsPlayer)>(player)
      .map_err(|_| hecs::NoSuchEntity)?;

    if delta == 0 {
      return Ok(());
    }

    let new_score = score.wrapping_add(delta as u32);
    let old_score = std::mem::replace(&mut score.0, new_score);
    if delta >= 0 {
      earnings.0 += delta as u32;
    }

    self.dispatch(PlayerScoreUpdate { player, old_score });

    Ok(())
  }

  /// Force an update packet for the player to be sent out within the next
  /// frame.
  ///
  /// Does nothing if the player doesn't exist.
  pub fn force_update(&self, player: Entity) {
    if let Ok(mut last_update) = self.world.get_mut::<LastUpdateTime>(player) {
      last_update.0 = self.start_time();
    }
  }

  /// Despawn an entity. This function takes care of dispatching the required
  /// events, deleting the entity, and creating a placeholder entity to prevent
  /// the entity id from being reused right away.
  ///
  /// Note that the placeholder entity is required to work around bugs in the
  /// airmash client.
  pub fn despawn(&mut self, entity: Entity) {
    use crate::event::EntityDespawn;

    if !self.world.contains(entity) {
      return;
    }

    self.dispatch(EntityDespawn { entity });
    let dispatch = self.dispatcher();
    dispatch.cleanup(self, move |game| {
      // HACK: By default spawn_at will reuse the same generation counter. However, we
      //       want to allocate a new entity that just has the same id. Hecs doesn't
      //       provide a way to do this so we take advantage of some internal
      //       implementation details in order to increment the generation ourselves.
      //
      //       There is a test at the end of this file that verifies that this works
      //       as expected.
      let entity = Entity::from_bits(entity.to_bits().get() + (1 << 32)).unwrap_or(entity);

      // The airmash client doesn't like it if you reuse ids soon after they get
      // destroyed. By reserving them for a minute we should prevent having dead
      // missiles just lying around.
      game.world.spawn_at(
        entity,
        (Expiry(Instant::now() + Duration::from_secs(10)), IsZombie),
      );
    });
  }
}

impl AirmashGame {
  /// Send a packet directly to a connection.
  ///
  /// This method is rather low-level. Generally you should be using one of the
  /// other methods for sending packets as they are more convenient.
  pub fn send_to_conn(&self, conn: ConnectionId, packet: impl Into<ServerPacket>) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(&packet.into()) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to_conn(conn, Arc::new(data));
  }

  /// Given an iterator of entities send the provided packet to all of them. If
  /// there are duplicate entities within the iterator then the packet will be
  /// sent multiple times to the corresponding connection.
  ///
  /// [`EntitySetBuilder`] is provided to help with building sets of entities.
  /// Alternatively, you can use the other `send_to_*` methods on this struct
  /// for common use-cases.
  pub fn send_to_entities<I>(&self, entities: I, packet: impl Into<ServerPacket>)
  where
    I: IntoIterator<Item = Entity>,
  {
    self._send_to_entities(entities, &packet.into());
  }
  fn _send_to_entities<I>(&self, entities: I, packet: &ServerPacket)
  where
    I: IntoIterator<Item = Entity>,
  {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(packet) {
      Ok(data) => Arc::new(data),
      Err(_) => return,
    };

    for entity in entities {
      connmgr.send_to(entity, Arc::clone(&data));
    }
  }

  /// Send a packet to the connection corresponding to the player.
  pub fn send_to(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self._send_to(player, &packet.into());
  }
  fn _send_to(&self, player: Entity, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to(player, Arc::new(data));
  }

  /// Send a packet to all players that are within the visible range of the
  /// provided position.
  pub fn send_to_visible(&self, pos: Vector2<f32>, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::visible(self, pos), packet);
  }

  /// Send a packet to all players on the provided team.
  pub fn send_to_team(&self, team: u16, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::team(self, team), packet)
  }

  /// Send a packet to all players on the provided team that are also within
  /// visible range of the provided position.
  pub fn send_to_team_visible(
    &self,
    team: u16,
    pos: Vector2<f32>,
    packet: impl Into<ServerPacket>,
  ) {
    self.send_to_entities(EntitySetBuilder::team_visible(self, team, pos), packet);
  }

  /// Send a packet to all players.
  pub fn send_to_all(&self, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::all(self), packet)
  }

  /// Send a packet to all players except the provided one.
  pub fn send_to_others(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::all(self).except(player), packet)
  }
}

/// Utility for building a set of players.
///
/// This is mainly intended for use with [`AirmashGame::send_to_entities`].
#[derive(Default)]
pub struct EntitySetBuilder {
  entries: HashSet<Entity>,
}

impl EntitySetBuilder {
  /// Create an empty entity set.
  pub fn empty() -> Self {
    Self::default()
  }

  /// Create an entity set with all players on `team` within the view radius of
  /// `pos`.
  pub fn team_visible(game: &AirmashGame, team: u16, pos: Vector2<f32>) -> Self {
    use crate::resource::collision::PlayerPosDb;

    let db = game.resources.read::<PlayerPosDb>();
    let config = game.resources.read::<Config>();

    let mut me = Self::default();
    db.query(
      pos,
      config.view_radius,
      LayerSpec::Include(team),
      &mut me.entries,
    );
    me
  }

  /// Create an entity set with all players within the view radius of `pos`.
  pub fn visible(game: &AirmashGame, pos: Vector2<f32>) -> Self {
    use crate::resource::collision::PlayerPosDb;

    let db = game.resources.read::<PlayerPosDb>();
    let config = game.resources.read::<Config>();

    let mut me = Self::default();
    db.query(pos, config.view_radius, LayerSpec::None, &mut me.entries);
    me
  }

  /// Create an entity set with all players on `req_team`.
  pub fn team(game: &AirmashGame, req_team: u16) -> Self {
    let mut query = game.world.query::<&Team>().with::<IsPlayer>();

    Self {
      entries: query
        .iter()
        .filter(|(_, team)| team.0 == req_team)
        .map(|(ent, _)| ent)
        .collect(),
    }
  }

  /// Create an entity set with all players in the game.
  pub fn all(game: &AirmashGame) -> Self {
    let mut query = game.world.query::<()>().with::<IsPlayer>();

    Self {
      entries: query.iter().map(|(ent, _)| ent).collect(),
    }
  }

  /// If `player` is contained within the entity set, then remove them.
  pub fn except(mut self, player: Entity) -> Self {
    self.entries.remove(&player);
    self
  }

  /// If `player` is not contained within the entity set then add them.
  pub fn including(mut self, player: Entity) -> Self {
    self.entries.insert(player);
    self
  }
}

impl IntoIterator for EntitySetBuilder {
  type Item = Entity;
  type IntoIter = <HashSet<Entity> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.entries.into_iter()
  }
}

#[test]
fn hecs_entity_has_id_in_lower_32_bits() {
  let ent1 = Entity::from_bits((77 << 32) + 22).unwrap();
  let ent2 = Entity::from_bits(ent1.to_bits().get() + (1 << 32)).unwrap();

  assert_eq!(ent1.id(), ent2.id());
}
