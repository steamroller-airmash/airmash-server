use specs::*;

use types::systemdata::*;
use types::*;

use systems::specials::prowler::SetStealth;
use SystemInfo;

use component::event::*;
use component::flag::ForcePlayerUpdate;
use utils::{EventHandler, EventHandlerTypeProvider};

use protocol::server::EventStealth;

#[derive(Default)]
pub struct SendEventStealth;

#[derive(SystemData)]
pub struct SendEventStealthData<'a> {
	conns: SendToVisible<'a>,

	pos: ReadStorage<'a, Position>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
	is_alive: IsAlive<'a>,
	force: WriteStorage<'a, ForcePlayerUpdate>,
}

impl EventHandlerTypeProvider for SendEventStealth {
	type Event = PlayerStealth;
}

impl<'a> EventHandler<'a> for SendEventStealth {
	type SystemData = SendEventStealthData<'a>;

	fn on_event(&mut self, evt: &PlayerStealth, data: &mut Self::SystemData) {
		let ref conns = data.conns;
		let ref pos = data.pos;
		let ref energy = data.energy;
		let ref energy_regen = data.energy_regen;
		let ref is_alive = data.is_alive;
		let ref mut force = data.force;

		if !is_alive.get(evt.player) {
			return;
		}

		let packet = EventStealth {
			id: evt.player.into(),
			state: evt.stealthed,
			energy: *try_get!(evt.player, energy),
			energy_regen: *try_get!(evt.player, energy_regen),
		};

		if evt.stealthed {
			let pos = *try_get!(evt.player, pos);
			conns.send_to_visible(pos, packet);
		} else {
			conns.send_to_player(evt.player, packet);

			force.insert(evt.player, ForcePlayerUpdate).unwrap();
		}
	}
}

impl SystemInfo for SendEventStealth {
	type Dependencies = SetStealth;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
