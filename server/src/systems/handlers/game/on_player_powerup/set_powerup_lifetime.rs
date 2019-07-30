use specs::*;

use crate::component::event::PlayerPowerup;
use crate::component::time::ThisFrame;
use crate::types::Powerups;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

#[derive(Default)]
pub struct SetPowerupLifetime;

#[derive(SystemData)]
pub struct SetPowerupLifetimeData<'a> {
	powerups: WriteStorage<'a, Powerups>,
	this_frame: Read<'a, ThisFrame>,
	entities: Entities<'a>,
}

impl EventHandlerTypeProvider for SetPowerupLifetime {
	type Event = PlayerPowerup;
}

impl<'a> EventHandler<'a> for SetPowerupLifetime {
	type SystemData = SetPowerupLifetimeData<'a>;

	fn on_event(&mut self, evt: &PlayerPowerup, data: &mut Self::SystemData) {
		if !data.entities.is_alive(evt.player) {
			return;
		}

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
