use specs::*;

use types::systemdata::*;
use types::*;

use systems::specials::prowler::SetStealth;
use SystemInfo;

use component::event::*;
use component::time::{LastUpdate, StartTime};
use utils::{EventHandler, EventHandlerTypeProvider};

use protocol::server::EventStealth;

#[derive(Default)]
pub struct SendEventStealth;

#[derive(SystemData)]
pub struct SendEventStealthData<'a> {
	pub conns: Read<'a, Connections>,
	pub start_time: Read<'a, StartTime>,

	pub pos: ReadStorage<'a, Position>,
	pub energy: ReadStorage<'a, Energy>,
	pub energy_regen: ReadStorage<'a, EnergyRegen>,
	pub is_alive: IsAlive<'a>,
	pub last_update: WriteStorage<'a, LastUpdate>,
}

impl EventHandlerTypeProvider for SendEventStealth {
	type Event = PlayerStealth;
}

impl<'a> EventHandler<'a> for SendEventStealth {
	type SystemData = SendEventStealthData<'a>;

	fn on_event(&mut self, evt: &PlayerStealth, data: &mut Self::SystemData) {
		let Self::SystemData {
			conns,
			start_time,

			pos,
			energy,
			energy_regen,
			is_alive,
			ref mut last_update,
		} = data;

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

			// Force position update system to send an update packet
			// by changing the time of the last update to the server
			// start time
			last_update
				.insert(evt.player, LastUpdate(start_time.0))
				.unwrap();
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
