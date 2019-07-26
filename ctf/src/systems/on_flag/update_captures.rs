use specs::*;

use crate::component::*;

use crate::server::utils::*;
use crate::server::*;

#[derive(Default)]
pub struct UpdateCaptures;

#[derive(SystemData)]
pub struct UpdateCapturesData<'a> {
	entities: Entities<'a>,
	captures: WriteStorage<'a, Captures>,
}

impl EventHandlerTypeProvider for UpdateCaptures {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for UpdateCaptures {
	type SystemData = UpdateCapturesData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		match evt.ty {
			FlagEventType::Capture => (),
			_ => return,
		};

		let player = evt.player.unwrap();

		if !data.entities.is_alive(player) {
			return;
		}

		try_get!(player, mut data.captures).0 += 1;
	}
}

impl SystemInfo for UpdateCaptures {
	// It doesn't matter too much when we handle this
	// it can happen the next frame
	type Dependencies = (super::KnownEventSources);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
