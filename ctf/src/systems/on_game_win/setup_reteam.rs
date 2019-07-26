use specs::*;

use crate::server::component::event::TimerEvent;
use crate::server::types::FutureDispatcher;
use crate::server::utils::*;
use crate::server::*;

use crate::component::*;
use crate::consts::RETEAM_TIMER;
use crate::systems::on_flag::CheckWin;
use std::time::Duration;

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
