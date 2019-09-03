use specs::prelude::*;

use crate::component::time::ThisFrame;
use crate::types::FutureDispatcher;

#[derive(Default)]
pub struct RunTimedFutures;

impl<'a> System<'a> for RunTimedFutures {
	type SystemData = (WriteExpect<'a, FutureDispatcher>, Read<'a, ThisFrame>);

	fn run(&mut self, (mut data, now): Self::SystemData) {
		data.exec_tasks(now.0);
	}
}

system_info! {
	impl SystemInfo for RunTimedFutures {
		type Dependencies = ();
	}
}
