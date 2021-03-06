use crate::server::*;
use specs::*;

use crate::component::*;

use crate::server::systems::handlers::game::on_join::AllJoinHandlers;
use crate::server::utils::*;

use super::PickupFlag;

#[derive(Default)]
pub struct FlagSpeed;

#[derive(SystemData)]
pub struct FlagSpeedSystemData<'a> {
	keystate: WriteStorage<'a, KeyState>,
}

impl EventHandlerTypeProvider for FlagSpeed {
	type Event = FlagEvent;
}

impl<'a> EventHandler<'a> for FlagSpeed {
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

system_info! {
	impl SystemInfo for FlagSpeed {
		type Dependencies = (PickupFlag, AllJoinHandlers);
	}
}
