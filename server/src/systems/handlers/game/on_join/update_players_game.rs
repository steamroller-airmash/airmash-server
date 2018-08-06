use specs::*;

use systems::handlers::packet::LoginHandler;
use SystemInfo;

use component::channel::*;
use component::counter::PlayersGame;
use consts::NUM_PLAYERS;

use std::sync::atomic::Ordering::Relaxed;

pub struct UpdatePlayersGame {
	reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct UpdatePlayersGameData<'a> {
	pub channel: Read<'a, OnPlayerJoin>,

	pub playersgame: Write<'a, PlayersGame>,
}

impl<'a> System<'a> for UpdatePlayersGame {
	type SystemData = UpdatePlayersGameData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			data.playersgame.0 += 1;
			NUM_PLAYERS.fetch_add(1, Relaxed);
		}
	}
}

impl SystemInfo for UpdatePlayersGame {
	type Dependencies = LoginHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
