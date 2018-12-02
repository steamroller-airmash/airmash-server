use specs::*;

use server::component::event::*;
use server::types::FutureDispatcher;
use server::utils::*;
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
pub struct SetGameActive;

#[derive(SystemData)]
pub struct SetGameActiveData<'a> {
	game_active: Write<'a, GameActive>,
	dispatcher: ReadExpect<'a, FutureDispatcher>,
}

impl EventHandlerTypeProvider for SetGameActive {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for SetGameActive {
	type SystemData = SetGameActiveData<'a>;

	fn on_event(&mut self, _: &GameWinEvent, data: &mut Self::SystemData) {
		data.game_active.0 = false;

		data.dispatcher
			.run_delayed(*TIMER_DURATION, |inst| TimerEvent {
				ty: *SET_GAME_ACTIVE,
				instant: inst,
				data: None,
			})
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
