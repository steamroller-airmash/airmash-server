use specs::*;

use types::*;

use SystemInfo;

use systems::handlers::packet::LoginHandler;

use component::channel::*;
use component::counter::*;

pub struct InitEarnings {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct InitEarningsData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

	pub earnings: WriteStorage<'a, Earnings>,
}

impl<'a> System<'a> for InitEarnings {
	type SystemData = InitEarningsData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.earnings.insert(evt.id, Earnings(Score(0))).unwrap();
		}
	}
}

impl SystemInfo for InitEarnings {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
