use specs::*;

use component::*;

use server::component::channel::*;
use server::*;

pub struct InitCaptures {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitCapturesData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub conns: Read<'a, Connections>,

	pub captures: WriteStorage<'a, Captures>,
}

impl InitCaptures {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for InitCaptures {
	type SystemData = InitCapturesData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.captures.insert(evt.id, Captures(0)).unwrap();
		}
	}
}

impl SystemInfo for InitCaptures {
	// It doesn't matter too much when we handle this
	// it can happen the next frame
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}
