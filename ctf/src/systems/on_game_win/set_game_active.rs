use specs::*;

use server::component::event::*;
use server::types::FutureDispatcher;
use server::*;

use component::*;
use config::*;
use consts::*;
use systems::on_flag::CheckWin;

use std::time::Duration;

lazy_static! {
	static ref TIMER_DURATION: Duration = *GAME_RESET_TIME + Duration::from_millis(100);
}

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
	dispatcher: ReadExpect<'a, FutureDispatcher>,
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

			data.dispatcher
				.run_delayed(*TIMER_DURATION, |inst| TimerEvent {
					ty: *SET_GAME_ACTIVE,
					instant: inst,
					data: None,
				})
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
