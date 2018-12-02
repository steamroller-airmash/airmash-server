use specs::*;

use server::component::event::TimerEvent;
use server::types::FutureDispatcher;
use server::utils::*;
use server::*;

use component::*;
use consts::GAME_START_TIMER;
use std::time::Duration;
use systems::on_flag::CheckWin;

#[derive(Default)]
pub struct SetupGameStart;

#[derive(SystemData)]
pub struct SetupGameStartData<'a> {
	future: ReadExpect<'a, FutureDispatcher>,
}

impl EventHandlerTypeProvider for SetupGameStart {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for SetupGameStart {
	type SystemData = SetupGameStartData<'a>;

	fn on_event(&mut self, _: &GameWinEvent, data: &mut Self::SystemData) {
		data.future
			.run_delayed(Duration::from_secs(60), move |inst| TimerEvent {
				ty: *GAME_START_TIMER,
				instant: inst,
				data: None,
			});
	}
}

impl SystemInfo for SetupGameStart {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
