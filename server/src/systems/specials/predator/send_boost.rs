use shrev::*;
use specs::*;

use types::systemdata::*;
use types::*;

use systems::specials::predator::SetBoostingFlag;
use SystemInfo;

use component::flag::{IsBoosting, IsPlayer};

use protocol::server::EventBoost;

pub struct SendEventBoost {
	pub dirty: BitSet,
	pub insert: Option<ReaderId<InsertedFlag>>,
	pub remove: Option<ReaderId<RemovedFlag>>,
}

#[derive(SystemData)]
pub struct SendEventBoostData<'a> {
	pub entities: Entities<'a>,
	pub conns: Read<'a, Connections>,

	pub boosting: ReadStorage<'a, IsBoosting>,
	pub pos: ReadStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub vel: ReadStorage<'a, Velocity>,
	pub energy: ReadStorage<'a, Energy>,
	pub energy_regen: ReadStorage<'a, EnergyRegen>,
	pub is_player: ReadStorage<'a, IsPlayer>,
	pub is_alive: IsAlive<'a>,

	pub clock: ReadClock<'a>,
}

impl SendEventBoost {
	pub fn new() -> Self {
		Self {
			dirty: Default::default(),
			insert: None,
			remove: None,
		}
	}

	fn send_packets<'a>(&self, data: &SendEventBoostData<'a>, boost: bool) {
		let clock = data.clock.get();

		(
			&*data.entities,
			&data.pos,
			&data.rot,
			&data.vel,
			&data.energy,
			&data.energy_regen,
			&self.dirty,
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

				data.conns.send_to_all(packet);
			});
	}
}

impl<'a> System<'a> for SendEventBoost {
	type SystemData = SendEventBoostData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		let mut storage: WriteStorage<IsBoosting> = WriteStorage::fetch(&res);

		self.insert = Some(storage.track_inserted());
		self.remove = Some(storage.track_removed());
	}

	fn run(&mut self, data: Self::SystemData) {
		self.dirty.clear();
		data.boosting
			.populate_inserted(&mut self.insert.as_mut().unwrap(), &mut self.dirty);
		self.send_packets(&data, true);

		self.dirty.clear();
		data.boosting
			.populate_removed(&mut self.remove.as_mut().unwrap(), &mut self.dirty);
		self.send_packets(&data, false);
	}
}

impl SystemInfo for SendEventBoost {
	type Dependencies = SetBoostingFlag;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}
