use specs::*;

use server::component::event::TimerEvent;
use server::types::FutureDispatcher;
use server::utils::*;
use server::*;

use component::*;
use consts::RETEAM_TIMER;
use std::time::Duration;
use systems::on_flag::CheckWin;

#[derive(Default)]
pub struct SetupReteam;

#[derive(SystemData)]
pub struct SetupReteamData<'a> {
	future: ReadExpect<'a, FutureDispatcher>,
}

impl EventHandlerTypeProvider for SetupReteam {
	type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for SetupReteam {
	type SystemData = SetupReteamData<'a>;

	fn on_event(&mut self, _: &GameWinEvent, data: &mut Self::SystemData) {
		data.future
			.run_delayed(Duration::from_secs(55), move |inst| TimerEvent {
				ty: *RETEAM_TIMER,
				instant: inst,
				data: None,
			});
	}
}

impl SystemInfo for SetupReteam {
	type Dependencies = CheckWin;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
