
use tokio::prelude::Sink;
use specs::prelude::*;
use specs::*;
use types::*;

pub struct PollComplete {
	
}

impl PollComplete {
	pub fn new() -> Self {
		Self { }
	}
}

impl<'a> System<'a> for PollComplete {
	type SystemData = Read<'a, Connections>;

	fn run(&mut self, conns: Self::SystemData) {
		for conn in conns.iter() {
			conn.sink.lock()
				.unwrap()
				.poll_complete()
				.unwrap();
		}
	}
}
