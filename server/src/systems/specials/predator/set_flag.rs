use specs::*;
use types::*;

use systems::specials::config::*;

use component::flag::IsBoosting;
use protocol::PlaneType;
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
		let mut clears = vec![];
		let mut keystate = data.keystate;

		(
			&data.plane,
			&data.energy,
			keystate.mask(),
			&mut data.energy_regen,
			&*data.entities,
		)
			.join()
			.filter(|(plane, _, _, _, _)| **plane == PlaneType::Predator)
			.for_each(|(_, energy, _, energy_regen, ent)| {
				let keystate = keystate.get(ent).unwrap();

				if *energy == Energy::new(0.0) || !keystate.special {
					if boosting.get(ent).is_some() {
						clears.push(ent);
						*energy_regen = info.energy_regen;

						boosting.remove(ent);
					}
				} else if keystate.special && (keystate.up || keystate.down) {
					*energy_regen = *PREDATOR_SPECIAL_REGEN;

					// Only insert when there is no value there
					// already, to prevent multiple change
					// flags from being set
					if boosting.get(ent).is_none() {
						boosting.insert(ent, IsBoosting).unwrap();
					}
				}
			});

		// Clear specific keys without iterating over
		// all key states mutably
		for ent in clears {
			keystate.get_mut(ent).unwrap().special = false;
		}
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
