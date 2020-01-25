use crate::ecs::prelude::*;
use crate::resource::builtin::{CurrentFrame, StartTime};
use crate::util::ToClock;

/// Get the number of clock ticks between
/// the start of the game and now.
///
/// This component adapter reads [`StartTime`][0].
///
/// [0]: crate::component::time::StartTime
#[derive(SystemData)]
pub struct ReadClock<'a> {
    start: ReadExpect<'a, StartTime>,
    frame: ReadExpect<'a, CurrentFrame>,
}

impl<'a> ReadClock<'a> {
    pub fn ticks(&self) -> u32 {
        (self.frame.0 - self.start.0).to_clock()
    }
}
