use std::time::Instant;

use airmash_protocol::{MobType, Vector2};
use hecs::{Entity, EntityBuilder, NoSuchEntity};
use smallvec::SmallVec;
use nalgebra::vector;

use crate::component::*;
use crate::event::PlayerFire;
use crate::{
  resource::{Config, LastFrame, ThisFrame},
  AirmashWorld,
};

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

      let mut builder = EntityBuilder::new();
      builder
        .add(Position(pos))
        .add(Velocity(vel))
        .add(info.ty)
        .add(IsMissile)
        .add(Owner(player))
        .add(team.0)
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
}
