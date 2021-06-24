use crate::resource::*;
use crate::{protocol::Time, AirmashWorld};
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
