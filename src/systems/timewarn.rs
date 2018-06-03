
use specs::*;
use types::*;

use std::time::{Duration, Instant};

pub struct TimeWarn{}

impl<'a> System<'a> for TimeWarn {
	type SystemData = (
		Read<'a, ThisFrame>,
		Read<'a, LastFrame>
	);

	fn run(&mut self, (this, last): Self::SystemData) {
		let millis = 18;
		if this.0 - last.0 > Duration::from_millis(millis) {
			warn!(
				target: "server",
				"Server frame exceeded {} ms, duration: {:?} ms",
				millis,
				((this.0 - last.0).subsec_nanos() as f64) * 0.000001
			);
		}
	}
}
