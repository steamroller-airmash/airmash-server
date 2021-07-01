#![allow(dead_code)]

use nalgebra::vector;

use crate::{protocol::*, FireMissileInfo};
use std::time::Duration;

mod hitcircles;
mod terrain;

pub use self::hitcircles::hitcircles_for_plane;
pub use self::terrain::TERRAIN;

/// The pred special causes negative energy regen this value is the rate at
/// which it causes energy to decrease.
pub const PREDATOR_SPECIAL_REGEN: EnergyRegen = -0.01;

pub const GOLIATH_SPECIAL_ENERGY: Energy = 0.0;
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
pub const TORNADO_MISSILE_DETAILS: [FireMissileInfo; 3] = [
  FireMissileInfo {
    pos_offset: vector![0.0, 40.1],
    rot_offset: 0.0,
    ty: MobType::TornadoTripleMissile,
  },
  FireMissileInfo {
    pos_offset: vector![15.0, 9.6],
    rot_offset: -0.05,
    ty: MobType::TornadoTripleMissile,
  },
  FireMissileInfo {
    pos_offset: vector![-15.0, 9.6],
    rot_offset: 0.05,
    ty: MobType::TornadoTripleMissile,
  },
];
pub const TORNADO_INFERNO_MISSILE_DETAILS: [FireMissileInfo; 5] = [
  FireMissileInfo {
    pos_offset: vector![0.0, 40.1],
    rot_offset: 0.0,
    ty: MobType::TornadoTripleMissile,
  },
  FireMissileInfo {
    pos_offset: vector![30.0, 15.0],
    rot_offset: -0.1,
    ty: MobType::TornadoTripleMissile,
  },
  FireMissileInfo {
    pos_offset: vector![20.0, 25.0],
    rot_offset: -0.05,
    ty: MobType::TornadoTripleMissile,
  },
  FireMissileInfo {
    pos_offset: vector![-20.0, 25.0],
    rot_offset: 0.05,
    ty: MobType::TornadoTripleMissile,
  },
  FireMissileInfo {
    pos_offset: vector![-30.0, 15.0],
    rot_offset: 0.1,
    ty: MobType::TornadoTripleMissile,
  },
];

pub const PROWLER_SPECIAL_ENERGY: Energy = 0.6;
pub const PROWLER_SPECIAL_DELAY: Duration = Duration::from_millis(1500);
