use specs::*;

use types::systemdata::*;
use types::*;

use systems::specials::prowler::SetStealth;
use SystemInfo;

use component::channel::*;
use component::time::{LastUpdate, StartTime};

use protocol::server::EventStealth;

pub struct SendEventStealth {
	pub reader: Option<OnPlayerStealthReader>,
}

#[derive(SystemData)]
pub struct SendEventStealthData<'a> {
	pub conns: Read<'a, Connections>,
	pub channel: Read<'a, OnPlayerStealth>,
	pub start_time: Read<'a, StartTime>,

	pub energy: ReadStorage<'a, Energy>,
	pub energy_regen: ReadStorage<'a, EnergyRegen>,
	pub is_alive: IsAlive<'a>,
	pub last_update: WriteStorage<'a, LastUpdate>,
}

impl SendEventStealth {
	pub fn new() -> Self {
		Self { reader: None }
	}
}

impl<'a> System<'a> for SendEventStealth {
	type SystemData = SendEventStealthData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnPlayerStealth>().register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			conns,
			channel,
			start_time,

			energy,
			energy_regen,
			is_alive,
			mut last_update,
		} = data;

		for evt in channel.read(self.reader.as_mut().unwrap()) {
			if !is_alive.get(evt.player) {
				continue;
			}

			let packet = EventStealth {
				id: evt.player.into(),
				state: evt.stealthed,
				energy: *energy.get(evt.player).unwrap(),
				energy_regen: *energy_regen.get(evt.player).unwrap(),
			};

			if evt.stealthed {
				conns.send_to_visible(evt.player, packet);
			} else {
				conns.send_to_player(evt.player, packet);

				// Force position update system to send an update packet
				// by changing the time of the last update to the server
				// start time
				*last_update.get_mut(evt.player).unwrap() = LastUpdate(start_time.0);
			}
		}
	}
}

impl SystemInfo for SendEventStealth {
	type Dependencies = SetStealth;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}
