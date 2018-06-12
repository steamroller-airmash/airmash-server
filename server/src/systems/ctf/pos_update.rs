
use specs::*;
use types::*;

use component::ctf::*;

pub struct PosUpdateSystem;

#[derive(SystemData)]
pub struct PosUpdateSystemData<'a> {
	pub ents: Entities<'a>,
	pub pos: WriteStorage<'a, Position>,
	pub flag: ReadStorage<'a, IsFlag>,
	pub carrier: ReadStorage<'a, FlagCarrier>
}

impl<'a> System<'a> for PosUpdateSystem {
	type SystemData = PosUpdateSystemData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let carriers = (&data.carrier, &*data.ents, &data.flag).join()
			.filter(|(c, _,  _)| c.0.is_some())
			.map(|(c, ent, _)| (c.0.unwrap(), ent))
			.collect::<Vec<(Entity, Entity)>>();

		for (carrier, flag) in carriers {
			// Update flag position to carrier position
			*data.pos.get_mut(flag).unwrap() = *data.pos.get(carrier).unwrap();
		}
	}
}
