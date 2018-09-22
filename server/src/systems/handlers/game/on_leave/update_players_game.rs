use specs::*;

use SystemInfo;

use component::channel::*;
use consts::NUM_PLAYERS;

use std::sync::atomic::Ordering::Relaxed;

pub struct UpdatePlayersGame {
	reader: Option<OnPlayerLeaveReader>,
}

#[derive(SystemData)]
pub struct UpdatePlayersGameData<'a> {
	pub channel: Read<'a, OnPlayerLeave>,
}

impl<'a> System<'a> for UpdatePlayersGame {
	type SystemData = UpdatePlayersGameData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerLeave>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			NUM_PLAYERS.fetch_sub(1, Relaxed);
		}
	}
}

impl SystemInfo for UpdatePlayersGame {
	type Dependencies = (super::KnownEventSources);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
