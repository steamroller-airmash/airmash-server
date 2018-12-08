use server::*;
use specs::*;

use component::*;

use server::systems::handlers::game::on_join::AllJoinHandlers;
use server::utils::*;

use super::PickupFlagSystem;

#[derive(Default)]
pub struct FlagSpeedSystem;

#[derive(SystemData)]
pub struct FlagSpeedSystemData<'a> {
	keystate: WriteStorage<'a, KeyState>,
}

impl EventHandlerTypeProvider for FlagSpeedSystem {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for FlagSpeedSystem {
	type SystemData = FlagSpeedSystemData<'a>;

	fn on_event(&mut self, evt: &FlagEvent, data: &mut Self::SystemData) {
		let ref mut keystate = data.keystate;

		let player = match evt.player {
			Some(x) => x,
			None => return,
		};
		let keystate = try_get!(player, mut keystate);

		match evt.ty {
			FlagEventType::Capture | FlagEventType::Drop => {
				keystate.flagspeed = false;
			}
			FlagEventType::PickUp => {
				keystate.flagspeed = true;
			}
			_ => (),
		}
	}
}

impl SystemInfo for FlagSpeedSystem {
	type Dependencies = (PickupFlagSystem, AllJoinHandlers);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
