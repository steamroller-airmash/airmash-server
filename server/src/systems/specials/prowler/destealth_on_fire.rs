use specs::*;
use types::*;

use component::channel::*;
use systems::missile::MissileFireHandler;
use SystemInfo;

use protocol::server::EventStealth;

pub struct DestealthOnFire {
	reader: Option<OnMissileFireReader>,
}

#[derive(SystemData)]
pub struct DestealthOnFireData<'a> {
	channel: Read<'a, OnMissileFire>,
	conns: Read<'a, Connections>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl<'a> System<'a> for DestealthOnFire {
	type SystemData = DestealthOnFireData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnMissileFire>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for evt in data.channel.read(self.reader.as_mut().unwrap()) {
			if *data.plane.get(evt.player).unwrap() != Plane::Prowler {
				continue;
			}

			data.keystate.get_mut(evt.player).unwrap().stealthed = false;

			let packet = EventStealth {
				id: evt.player.into(),
				state: false,
				energy: *data.energy.get(evt.player).unwrap(),
				energy_regen: *data.energy_regen.get(evt.player).unwrap(),
			};

			data.conns.send_to_player(evt.player, packet);
		}
	}
}

impl SystemInfo for DestealthOnFire {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
