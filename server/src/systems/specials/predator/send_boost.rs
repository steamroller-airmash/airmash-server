use shrev::*;
use specs::prelude::ComponentEvent;
use specs::*;

use types::systemdata::*;
use types::*;
use utils::maybe_init::MaybeInit;

use systems::specials::predator::SetBoostingFlag;
use SystemInfo;

use component::flag::{IsBoosting, IsPlayer};

use protocol::server::EventBoost;

#[derive(Default)]
pub struct SendEventBoost {
	pub inserted: BitSet,
	pub removed: BitSet,
	pub changed: MaybeInit<ReaderId<ComponentEvent>>,
}

#[derive(SystemData)]
pub struct SendEventBoostData<'a> {
	entities: Entities<'a>,
	conns: Read<'a, Connections>,

	boosting: ReadStorage<'a, IsBoosting>,
	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
	vel: ReadStorage<'a, Velocity>,
	energy: ReadStorage<'a, Energy>,
	energy_regen: ReadStorage<'a, EnergyRegen>,
	is_player: ReadStorage<'a, IsPlayer>,
	is_alive: IsAlive<'a>,

	clock: ReadClock<'a>,
}

impl SendEventBoost {
	fn send_packets<'a>(&self, data: &SendEventBoostData<'a>, boost: bool, dirty: &BitSet) {
		let clock = data.clock.get();

		(
			&*data.entities,
			&data.pos,
			&data.rot,
			&data.vel,
			&data.energy,
			&data.energy_regen,
			dirty,
			data.is_alive.mask(),
			data.is_player.mask(),
		)
			.join()
			.for_each(|(ent, pos, rot, vel, energy, energy_regen, _, _, _)| {
				let packet = EventBoost {
					id: ent.into(),
					clock: clock,
					boost: boost,

					pos: *pos,
					rot: *rot,
					speed: *vel,

					energy: *energy,
					energy_regen: *energy_regen,
				};

				data.conns.send_to_visible(*pos, packet);
			});
	}
}

impl<'a> System<'a> for SendEventBoost {
	type SystemData = SendEventBoostData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		let mut storage: WriteStorage<IsBoosting> = WriteStorage::fetch(&res);

		self.changed = MaybeInit::new(storage.register_reader());
	}

	fn run(&mut self, data: Self::SystemData) {
		self.inserted.clear();
		self.removed.clear();

		for event in data.boosting.channel().read(&mut self.changed) {
			match event {
				ComponentEvent::Inserted(id) => {
					self.inserted.add(*id);
				}
				ComponentEvent::Removed(id) => {
					self.removed.add(*id);
				}
				_ => (),
			}
		}

		self.send_packets(&data, true, &self.inserted);
		self.send_packets(&data, false, &self.removed);
	}
}

impl SystemInfo for SendEventBoost {
	type Dependencies = SetBoostingFlag;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}
