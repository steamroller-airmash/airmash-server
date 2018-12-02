use specs::*;

use server::component::event::*;
use server::utils::*;
use server::*;

use component::*;
use consts::*;

/// Resets game score to 0-0 when the
/// game starts.
#[derive(Default)]
pub struct SetGameActive;

#[derive(SystemData)]
pub struct SetGameActiveData<'a> {
	game_active: Write<'a, GameActive>,
}

impl EventHandlerTypeProvider for SetGameActive {
	type Event = TimerEvent;
}

impl<'a> EventHandler<'a> for SetGameActive {
	type SystemData = SetGameActiveData<'a>;

	fn on_event(&mut self, evt: &TimerEvent, data: &mut Self::SystemData) {
		if evt.ty != *SET_GAME_ACTIVE {
			return;
		}

		data.game_active.0 = true;
	}
}

impl SystemInfo for SetGameActive {
	type Dependencies = ();

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
