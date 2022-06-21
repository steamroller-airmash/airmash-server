//! Various utility functions that perform common functions.

use std::time::{Duration, Instant};

use nalgebra::vector;

use crate::component::{Effects, Upgrades};
use crate::protocol::{Time, Vector2};
use crate::resource::*;
use crate::AirmashGame;

pub(crate) mod escapes;
mod powerup_spawner;
pub(crate) mod serde;
pub mod spectate;

pub use self::powerup_spawner::PeriodicPowerupSpawner;

pub fn convert_time(dur: Duration) -> Time {
  dur.as_secs_f32() * 60.0
}

pub fn get_time_clock(game: &AirmashGame, time: Instant) -> u32 {
  let start_time = game
    .resources
    .get::<StartTime>()
    .expect("StartTime not registered in resources");

  let duration = time.saturating_duration_since(start_time.0);
  (((duration.as_secs() * 1_000_000) + duration.subsec_micros() as u64) / 10) as u32
}

pub fn get_current_clock(game: &AirmashGame) -> u32 {
  let this_frame = game
    .resources
    .get::<ThisFrame>()
    .expect("ThisFrame not registered in resources");

  get_time_clock(game, this_frame.0)
}

pub fn get_server_upgrades(upgrades: &Upgrades, effects: &Effects) -> crate::protocol::Upgrades {
  crate::protocol::Upgrades {
    speed: upgrades.speed,
    shield: effects.has_shield(),
    inferno: effects.has_inferno(),
  }
}

pub fn rotate(v: Vector2<f32>, angle: f32) -> Vector2<f32> {
  let (sin, cos) = angle.sin_cos();
  vector![v.x * cos - v.y * sin, v.x * sin + v.y * cos]
}
