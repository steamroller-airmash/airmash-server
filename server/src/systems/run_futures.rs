use specs::*;

use SystemInfo;

use component::time::ThisFrame;
use types::FutureDispatcher;

pub struct RunTimedFutures;

impl<'a> System<'a> for RunTimedFutures {
	type SystemData = (WriteExpect<'a, FutureDispatcher>, Read<'a, ThisFrame>);

	fn run(&mut self, (mut data, now): Self::SystemData) {
		data.exec_tasks(now.0);
	}
}

impl SystemInfo for RunTimedFutures {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
