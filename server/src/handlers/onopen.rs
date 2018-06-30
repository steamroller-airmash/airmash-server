use shrev::*;
use specs::*;
use types::*;

use std::mem;

use dispatch::SystemInfo;
use types::event::ConnectionOpen;

use systems::PacketHandler;

pub struct OnOpenHandler {
	reader: Option<ReaderId<ConnectionOpen>>,
}

impl OnOpenHandler {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for OnOpenHandler {
	type SystemData = (
		Read<'a, EventChannel<ConnectionOpen>>,
		Write<'a, Connections>,
	);

	fn setup(&mut self, res: &mut Resources) {
		self.reader = Some(
			res.fetch_mut::<EventChannel<ConnectionOpen>>()
				.register_reader(),
		);

		Self::SystemData::setup(res);
	}

	fn run(&mut self, (channel, mut connections): Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for evt in channel.read(reader) {
				let sink = mem::replace(&mut *evt.sink.lock().unwrap(), None);

				connections.add(evt.conn, sink.unwrap(), evt.addr, evt.origin.clone());
			}
		}
	}
}

impl SystemInfo for OnOpenHandler {
	type Dependencies = PacketHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		module_path!()
	}
}
