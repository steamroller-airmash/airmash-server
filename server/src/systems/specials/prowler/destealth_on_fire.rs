use specs::*;
use types::*;

use component::event::*;
use systems::missile::MissileFireHandler;
use SystemInfo;

use utils::{EventHandler, EventHandlerTypeProvider};

use protocol::server::EventStealth;

#[derive(Default)]
pub struct DestealthOnFire;

#[derive(SystemData)]
pub struct DestealthOnFireData<'a> {
	conns: Read<'a, Connections>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl EventHandlerTypeProvider for DestealthOnFire {
	type Event = MissileFire;
}

impl<'a> EventHandler<'a> for DestealthOnFire {
	type SystemData = DestealthOnFireData<'a>;

	fn on_event(&mut self, evt: &MissileFire, data: &mut Self::SystemData) {
		if *try_get!(evt.player, data.plane) != Plane::Prowler {
			return;
		}

		try_get!(evt.player, mut data.keystate).stealthed = false;

		let packet = EventStealth {
			id: evt.player.into(),
			state: false,
			energy: *try_get!(evt.player, data.energy),
			energy_regen: *try_get!(evt.player, data.energy_regen),
		};

		data.conns.send_to_player(evt.player, packet);
	}
}

impl SystemInfo for DestealthOnFire {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
