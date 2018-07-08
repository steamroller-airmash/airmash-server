use specs::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::channel::*;
use component::time::{LastStealthTime, StartTime};

pub struct InitStealthTime {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitStealthTimeData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,
	pub start_time: Read<'a, StartTime>,

	pub last_stealth: WriteStorage<'a, LastStealthTime>,
}

impl<'a> System<'a> for InitStealthTime {
	type SystemData = InitStealthTimeData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.last_stealth
				.insert(evt.id, LastStealthTime(data.start_time.0))
				.unwrap();
		}
	}
}

impl SystemInfo for InitStealthTime {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
