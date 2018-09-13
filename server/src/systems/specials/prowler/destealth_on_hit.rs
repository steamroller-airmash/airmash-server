use specs::*;
use types::systemdata::*;
use types::*;

use component::channel::*;
use component::event::*;
use component::flag::*;
use systems::collision::PlayerMissileCollisionSystem;
use SystemInfo;

use protocol::server::EventStealth;

pub struct DestealthOnHit {
	reader: Option<OnPlayerMissileCollisionReader>,
}

#[derive(SystemData)]
pub struct DestealthOnHitData<'a> {
	channel: Read<'a, OnPlayerMissileCollision>,
	conns: Read<'a, Connections>,

	keystate: WriteStorage<'a, KeyState>,
	plane: ReadStorage<'a, Plane>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl<'a> System<'a> for DestealthOnHit {
	type SystemData = DestealthOnHitData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(
			res.fetch_mut::<OnPlayerMissileCollision>()
				.register_reader(),
		);
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for PlayerMissileCollision(evt) in data.channel.read(self.reader.as_mut().unwrap()) {
			let player = data
				.is_player
				.get(evt.0.ent)
				.map(|_| evt.0.ent)
				.unwrap_or(evt.1.ent);

			if *data.plane.get(player).unwrap() != Plane::Prowler {
				continue;
			}
			if !data.is_alive.get(player) {
				continue;
			}

			data.keystate.get_mut(player).unwrap().stealthed = false;

			let packet = EventStealth {
				id: player.into(),
				state: false,
				energy: *data.energy.get(player).unwrap(),
				energy_regen: *data.energy_regen.get(player).unwrap(),
			};

			data.conns.send_to_player(player, packet);
		}
	}
}

impl SystemInfo for DestealthOnHit {
	type Dependencies = PlayerMissileCollisionSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self { reader: None }
	}
}
