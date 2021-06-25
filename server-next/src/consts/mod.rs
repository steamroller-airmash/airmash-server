#![allow(dead_code)]

use std::time::Duration;
use crate::protocol::*;

mod terrain;
mod hitcircles;

pub use self::terrain::TERRAIN;
pub use self::hitcircles::hitcircles_for_plane;

/// The pred special causes negative energy regen this value is the rate at
/// which it causes energy to decrease.
pub const PREDATOR_SPECIAL_REGEN: EnergyRegen = 0.01;

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

pub const PROWLER_SPECIAL_ENERGY: Energy = 0.6;
pub const PROWLER_SPECIAL_DELAY: Duration = Duration::from_millis(1500);
