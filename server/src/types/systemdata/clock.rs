use specs::*;

use std::time::Instant;

use crate::component::time::StartTime;
use crate::types::ToClock;

/// Get the number of clock ticks between
/// the start of the game and now.
///
/// This component adapter reads [`StartTime`][0].
///
/// [0]: ::component::time::StartTime
#[derive(SystemData)]
pub struct ReadClock<'a> {
  start: Read<'a, StartTime>,
}

impl<'a> ReadClock<'a> {
  pub fn get(&self) -> u32 {
    (Instant::now() - self.start.0).to_clock()
  }
}
