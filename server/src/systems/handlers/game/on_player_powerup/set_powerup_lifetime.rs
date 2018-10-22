use specs::*;

use component::event::PlayerPowerup;
use component::time::ThisFrame;
use types::Powerups;
use utils::{EventHandler, EventHandlerTypeProvider};
use SystemInfo;

#[derive(Default)]
pub struct SetPowerupLifetime;

#[derive(SystemData)]
pub struct SetPowerupLifetimeData<'a> {
	powerups: WriteStorage<'a, Powerups>,
	this_frame: Read<'a, ThisFrame>,
}

impl EventHandlerTypeProvider for SetPowerupLifetime {
	type Event = PlayerPowerup;
}

impl<'a> EventHandler<'a> for SetPowerupLifetime {
	type SystemData = SetPowerupLifetimeData<'a>;

	fn on_event(&mut self, evt: &PlayerPowerup, data: &mut Self::SystemData) {
		data.powerups
			.insert(
				evt.player,
				Powerups {
					ty: evt.ty,
					end_time: data.this_frame.0 + evt.duration,
				},
			)
			.unwrap();
	}
}

impl SystemInfo for SetPowerupLifetime {
	type Dependencies = super::KnownEventSources;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
