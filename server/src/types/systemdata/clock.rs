use specs::*;

use types::ToClock;

use component::time::{StartTime, ThisFrame};

#[derive(SystemData)]
pub struct ReadClock<'a> {
	pub start: Read<'a, StartTime>,
	pub frame: Read<'a, ThisFrame>,
}

impl<'a> ReadClock<'a> {
	pub fn get(&self) -> u32 {
		(self.frame.0 - self.start.0).to_clock()
	}
}
