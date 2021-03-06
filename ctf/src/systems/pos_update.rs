use crate::server::*;
use specs::*;

use crate::component::*;
use crate::server::types::systemdata::*;

#[derive(Default)]
pub struct PosUpdate;

#[derive(SystemData)]
pub struct PosUpdateData<'a> {
	ents: Entities<'a>,
	pos: WriteStorage<'a, Position>,
	flag: ReadStorage<'a, IsFlag>,
	carrier: ReadStorage<'a, FlagCarrier>,
	is_alive: IsAlive<'a>,
}

impl<'a> System<'a> for PosUpdate {
	type SystemData = PosUpdateData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let carriers = (&data.carrier, &*data.ents, &data.flag, data.is_alive.mask())
			.join()
			.filter(|(c, ..)| c.0.is_some())
			.map(|(c, ent, ..)| (ent, c.0.unwrap()));

		for (flag, carrier) in carriers {
			// Update flag position to carrier position
			let pos = *try_get!(carrier, data.pos);
			data.pos.insert(flag, pos).unwrap();
		}
	}
}

system_info! {
	impl SystemInfo for PosUpdate {
		type Dependencies = super::PickupFlag;
	}
}
