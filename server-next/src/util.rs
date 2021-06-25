use crate::component::Powerup;
use crate::component::Upgrades;
use crate::protocol::{PowerupType, Time};
use crate::resource::*;
use crate::AirmashWorld;
use std::time::Duration;

pub fn convert_time(dur: Duration) -> Time {
  dur.as_secs_f32() * 60.0
}

pub fn get_current_clock(game: &AirmashWorld) -> u32 {
  let start_time = game
    .resources
    .get::<StartTime>()
    .expect("StartTime not registered in resources");
  let this_frame = game
    .resources
    .get::<ThisFrame>()
    .expect("ThisFrame not registered in resources");

  let duration = this_frame.0 - start_time.0;
  (((duration.as_secs() * 1_000_000) + duration.subsec_micros() as u64) / 10) as u32
}

pub fn get_server_upgrades(
  upgrades: &Upgrades,
  powerup: Option<&Powerup>,
) -> crate::protocol::Upgrades {
  crate::protocol::Upgrades {
    speed: upgrades.speed,
    shield: powerup
      .map(|x| x.ty == PowerupType::Shield)
      .unwrap_or(false),
    inferno: powerup
      .map(|x| x.ty == PowerupType::Inferno)
      .unwrap_or(false),
  }
}
