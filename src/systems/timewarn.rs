
use specs::*;
use types::*;

use std::time::Duration;

pub struct TimeWarn{}

impl<'a> System<'a> for TimeWarn {
	type SystemData = (
		Read<'a, LastFrame>,
		Read<'a, ThisFrame>
	);

	fn run(&mut self, (last, this): Self::SystemData) {
		if last.0 - this.0 > Duration::from_millis(16) {
			warn!(
				target: "server",
				"Server frame exceeded 16ms, duration: {:?}",
				last.0 - this.0
			);
		}
	}
}
