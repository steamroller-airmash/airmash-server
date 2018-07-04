use specs::*;
use types::*;

use systems::specials::config::*;

use protocol::PlaneType;
use component::flag::IsBoosting;
use systems::handlers::packet::KeyHandler;
use SystemInfo;

pub struct SetBoostingFlag;

#[derive(SystemData)]
pub struct SetBoostingFlagData<'a> {
	pub config: Read<'a, Config>,
	pub entities: Entities<'a>,

	pub plane: ReadStorage<'a, Plane>,
	pub energy: ReadStorage<'a, Energy>,
	pub boosting: WriteStorage<'a, IsBoosting>,

	pub keystate: WriteStorage<'a, KeyState>,
	pub energy_regen: WriteStorage<'a, EnergyRegen>,
}

impl<'a> System<'a> for SetBoostingFlag {
	type SystemData = SetBoostingFlagData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let ref info = data.config.planes[PlaneType::Predator];
		let mut boosting = data.boosting;

		(
			&data.plane,
			&data.energy,
			&mut data.keystate,
			&mut data.energy_regen,
			&*data.entities,
		).join()
			.filter(|(plane, _, _, _, _)| **plane == PlaneType::Predator)
			.for_each(|(_, energy, keystate, energy_regen, ent)| {
				if *energy == Energy::new(0.0) || !keystate.special {
					keystate.special = false;
					*energy_regen = info.energy_regen;

					if boosting.get(ent).is_some() {
						boosting.remove(ent);
					}
				} else if keystate.special {
					*energy_regen = *PREDATOR_SPECIAL_REGEN;

					// Only insert when there is no value there 
					// already, to prevent multiple change
					// flags from being set
					if boosting.get(ent).is_none() {
						boosting.insert(ent, IsBoosting).unwrap();
					}
				}
				else {
					panic!();
				}
			});
	}
}

impl SystemInfo for SetBoostingFlag {
	type Dependencies = (KeyHandler);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
