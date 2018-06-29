
use specs::*;

use SystemInfo;
use std::any::Any;

use systems::handlers::packet::LoginHandler;

use component::channel::*;
use component::counter::*;

pub struct InitKillCounters {
	reader: Option<OnPlayerJoinReader>
}

#[derive(SystemData)]
pub struct InitKillCountersData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

	pub total_kills: WriteStorage<'a, TotalKills>,
	pub total_deaths: WriteStorage<'a, TotalDeaths>,
}

impl<'a> System<'a> for InitKillCounters {
	type SystemData = InitKillCountersData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerJoin>().register_reader()
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			data.total_kills.insert(evt.0, TotalKills(0)).unwrap();
			data.total_deaths.insert(evt.0, TotalDeaths(0)).unwrap();
		}
	}
}

impl SystemInfo for InitKillCounters {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new(_: Box<Any>) -> Self {
		Self{ reader: None }
	}
}

