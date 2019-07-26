use specs::*;

use crate::server::component::event::*;
use crate::server::utils::*;
use crate::server::*;

use crate::component::*;
use crate::consts::*;

/// Routes the [`GAME_START_TIMER`] into a separate
/// event ([`OnGameStart`]).
#[derive(Default)]
pub struct GameStart;

#[derive(SystemData)]
pub struct GameStartData<'a> {
	game_start_channel: Write<'a, OnGameStart>,
}

impl EventHandlerTypeProvider for GameStart {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for GameStart {
	type SystemData = GameStartData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *GAME_START_TIMER {
			return;
		}

		data.game_start_channel.single_write(GameStartEvent);
	}
}

impl SystemInfo for GameStart {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
