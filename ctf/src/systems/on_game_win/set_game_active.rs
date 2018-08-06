use server::*;
use specs::*;

use component::*;
use systems::on_flag::CheckWin;

/// Change GameActive state to false.
///
/// This is required to change game
/// logic based on whether a game is
/// currently running.
#[derive(Default)]
pub struct SetGameActive {
	reader: Option<OnGameWinReader>,
}

#[derive(SystemData)]
pub struct SetGameActiveData<'a> {
	channel: Read<'a, OnGameWin>,
	game_active: Write<'a, GameActive>,
}

impl<'a> System<'a> for SetGameActive {
	type SystemData = SetGameActiveData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameWin>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			data.game_active.0 = false;
		}
	}
}

impl SystemInfo for SetGameActive {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
