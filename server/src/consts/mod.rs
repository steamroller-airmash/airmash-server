#![allow(dead_code)]

use std::time::Duration;

use crate::config::MissilePrototypeRef;
use crate::protocol::*;
use crate::{FireMissileInfo, Vector2};

mod hitcircles;
mod terrain;

pub use self::hitcircles::hitcircles_for_plane;
pub use self::terrain::TERRAIN;

pub const UPGRADE_MULTIPLIERS: [f32; 6] = [1.0, 1.05, 1.1, 1.15, 1.2, 1.25];

/// The probability that, when an unupgraded player dies, they will drop an
/// upgrade.
pub const UPGRADE_DROP_PROBABILITY: f32 = 0.5;
/// The collision radius of a mob.
pub const MOB_COLLIDE_RADIUS: f32 = 10.0;

/// The pred special causes negative energy regen this value is the rate at
/// which it causes energy to decrease.
pub const PREDATOR_SPECIAL_REGEN: EnergyRegen = -0.01;

pub const GOLIATH_SPECIAL_ENERGY: Energy = 0.5;
// TODO: Replace this with real value (see issue #2)
/// The distance out to which a goliath repel has an effect
pub const GOLIATH_SPECIAL_RADIUS_MISSILE: Distance = 225.0;
pub const GOLIATH_SPECIAL_RADIUS_PLAYER: Distance = 180.0;
/// The speed at which players and mobs will be going when they are reflected.
pub const GOLIATH_SPECIAL_REFLECT_SPEED: Speed = 0.5;
/// Minimum time between reflects.
pub const GOLIATH_SPECIAL_INTERVAL: Duration = Duration::from_secs(1);

// TODO: Tornado
pub const TORNADO_SPECIAL_ENERGY: Energy = 0.9;
pub fn tornado_missile_details(proto: MissilePrototypeRef) -> [FireMissileInfo; 3] {
  [
    FireMissileInfo {
      pos_offset: Vector2::new(0.0, 40.1),
      rot_offset: 0.0,
      proto,
    },
    FireMissileInfo {
      pos_offset: Vector2::new(15.0, 9.6),
      rot_offset: -0.05,
      proto,
    },
    FireMissileInfo {
      pos_offset: Vector2::new(-15.0, 9.6),
      rot_offset: 0.05,
      proto,
    },
  ]
}
pub fn tornado_inferno_missile_details(proto: MissilePrototypeRef) -> [FireMissileInfo; 5] {
  [
    FireMissileInfo {
      pos_offset: Vector2::new(0.0, 40.1),
      rot_offset: 0.0,
      proto,
    },
    FireMissileInfo {
      pos_offset: Vector2::new(30.0, 15.0),
      rot_offset: -0.1,
      proto,
    },
    FireMissileInfo {
      pos_offset: Vector2::new(20.0, 25.0),
      rot_offset: -0.05,
      proto,
    },
    FireMissileInfo {
      pos_offset: Vector2::new(-20.0, 25.0),
      rot_offset: 0.05,
      proto,
    },
    FireMissileInfo {
      pos_offset: Vector2::new(-30.0, 15.0),
      rot_offset: 0.1,
      proto,
    },
  ]
}

pub const PROWLER_SPECIAL_ENERGY: Energy = 0.6;
pub const PROWLER_SPECIAL_DELAY: Duration = Duration::from_millis(1500);
