use std::time::Duration;
use std::time::Instant;

use airmash_protocol::{MobType, Vector2};
use hecs::{Entity, EntityBuilder, NoSuchEntity};
use nalgebra::vector;
use smallvec::SmallVec;

use crate::component::IsPlayer;
use crate::component::*;
use crate::event::EntitySpawn;
use crate::event::PlayerFire;
use crate::network::{ConnectionId, ConnectionMgr};
use crate::protocol::{v5, ServerPacket};
use crate::{
  resource::{Config, LastFrame, ThisFrame},
  AirmashWorld,
};

#[derive(Clone, Copy, Debug)]
pub struct FireMissileInfo {
  /// Starting offset of the missile, relative to the plane that is firing it.
  /// This will be rotated into the plane's frame of reference.
  pub pos_offset: Vector2<f32>,
  /// Direction that the missile will accelerate in, relative to the direction
  /// the plane is facing when it fires
  pub rot_offset: f32,
  /// Type of the missile
  pub ty: MobType,
}

impl AirmashWorld {
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

  /// Get the entity corresponding to the provided id.
  ///
  /// TODO: Currently this performs a linear scan on all entities. It should be
  /// accelerated with a hashmap.
  pub fn get_entity_by_id(&self, id: u16) -> Option<Entity> {
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

  pub fn despawn(&mut self, entity: Entity) {
    use crate::event::EntityDespawn;

    if !self.world.contains(entity) {
      return;
    }

    self.dispatch(EntityDespawn { entity });
    let dispatch = self.dispatcher.clone();
    dispatch.cleanup(self, move |game| {
      // The airmash client doesn't like it if you reuse ids soon after they get
      // destroyed. By reserving them for a minute we should prevent having dead
      // missiles just lying around.
      game.world.spawn_at(
        entity,
        (Expiry(Instant::now() + Duration::from_secs(60)), IsZombie),
      );
    });
  }
}

impl AirmashWorld {
  pub fn send_to_conn(&self, conn: ConnectionId, packet: impl Into<ServerPacket>) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(&packet.into()) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to_conn(conn, data);
  }

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
      Ok(data) => data,
      Err(_) => return,
    };

    for entity in entities {
      connmgr.send_to(entity, data.clone());
    }
  }

  pub fn send_to(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self._send_to(player, &packet.into());
  }
  fn _send_to(&self, player: Entity, packet: &ServerPacket) {
    let mut connmgr = self.resources.write::<ConnectionMgr>();
    let data = match v5::serialize(packet) {
      Ok(data) => data,
      Err(_) => return,
    };

    connmgr.send_to(player, data);
  }

  pub fn send_to_visible(&self, pos: Vector2<f32>, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::visible(self, None, pos), packet);
  }

  pub fn send_to_team(&self, team: u16, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::team(self, team), packet)
  }

  pub fn send_to_team_visible(
    &self,
    team: u16,
    pos: Vector2<f32>,
    packet: impl Into<ServerPacket>,
  ) {
    self.send_to_entities(EntitySetBuilder::visible(self, Some(team), pos), packet);
  }

  pub fn send_to_all(&self, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::all(self), packet)
  }

  pub fn send_to_others(&self, player: Entity, packet: impl Into<ServerPacket>) {
    self.send_to_entities(EntitySetBuilder::all(self).except(player), packet)
  }
}

#[derive(Default)]
pub struct EntitySetBuilder {
  entries: SmallVec<[Entity; 64]>,
}

impl EntitySetBuilder {
  pub fn visible(game: &AirmashWorld, team: Option<u16>, pos: Vector2<f32>) -> Self {
    use crate::resource::collision::PlayerPosDb;

    let db = game.resources.read::<PlayerPosDb>();
    let config = game.resources.read::<Config>();

    let mut me = Self::default();
    db.query(pos, config.view_radius, team, &mut me.entries);
    me
  }

  pub fn team(game: &AirmashWorld, req_team: u16) -> Self {
    let mut query = game.world.query::<&Team>().with::<IsPlayer>();

    Self {
      entries: query
        .iter()
        .filter(|(_, team)| team.0 == req_team)
        .map(|(ent, _)| ent)
        .collect(),
    }
  }

  pub fn all(game: &AirmashWorld) -> Self {
    let mut query = game.world.query::<()>().with::<IsPlayer>();

    Self {
      entries: query.iter().map(|(ent, _)| ent).collect(),
    }
  }

  pub fn except(mut self, player: Entity) -> Self {
    let index = self
      .entries
      .iter()
      .enumerate()
      .find(|(_, x)| **x == player)
      .map(|(i, _)| i);
    if let Some(index) = index {
      self.entries.swap_remove(index);
    }

    self
  }

  pub fn including(mut self, player: Entity) -> Self {
    self = self.except(player);
    self.entries.push(player);
    self
  }
}

impl IntoIterator for EntitySetBuilder {
  type Item = Entity;
  type IntoIter = <SmallVec<[Entity; 64]> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.entries.into_iter()
  }
}
