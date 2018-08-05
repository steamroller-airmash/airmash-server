use specs::*;

use super::*;
use server::*;

use component::*;
use systems::timer::GameStart;

/// Resets game score to 0-0 when the
/// game starts.
#[derive(Default)]
pub struct SetGameActive {
	reader: Option<OnGameStartReader>,
}

#[derive(SystemData)]
pub struct SetGameActiveData<'a> {
	channel: Read<'a, OnGameStart>,
	game_active: Write<'a, GameActive>,
}

impl<'a> System<'a> for SetGameActive {
	type SystemData = SetGameActiveData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameStart>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			data.game_active.0 = true;
		}
	}
}

impl SystemInfo for SetGameActive {
	type Dependencies = (
		GameStart,
		// We depend on RespawnAll so that a player can't
		// sit on the flag and get a cap between when this
		// system runs and when RespawnAll runs. Depending
		// on score reset order this could also result in
		// the game thinking that another win has occurred.
		RespawnAll,
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
