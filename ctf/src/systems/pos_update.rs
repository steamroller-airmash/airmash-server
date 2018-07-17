use server::*;
use specs::*;

use component::*;
use server::types::systemdata::*;

pub struct PosUpdateSystem;

#[derive(SystemData)]
pub struct PosUpdateSystemData<'a> {
	pub ents: Entities<'a>,
	pub pos: WriteStorage<'a, Position>,
	pub flag: ReadStorage<'a, IsFlag>,
	pub carrier: ReadStorage<'a, FlagCarrier>,
	pub is_alive: IsAlive<'a>,
}

impl<'a> System<'a> for PosUpdateSystem {
	type SystemData = PosUpdateSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let carriers = (&data.carrier, &*data.ents, &data.flag, data.is_alive.mask())
			.join()
			.filter(|(c, ..)| c.0.is_some())
			.map(|(c, ent, ..)| (ent, c.0.unwrap()));

		for (flag, carrier) in carriers {
			// Update flag position to carrier position
			*data.pos.get_mut(flag).unwrap() = *data.pos.get(carrier).unwrap();
		}
	}
}

use super::PickupFlagSystem;

impl SystemInfo for PosUpdateSystem {
	type Dependencies = PickupFlagSystem;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
