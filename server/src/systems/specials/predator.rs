
use specs::*;
use types::*;

use super::config::*;

use SystemInfo;
use protocol::PlaneType;
use systems::handlers::packet::KeyHandler;

pub struct PredatorSpecial;

#[derive(SystemData)]
pub struct PredatorSpecialData<'a> {
	pub config: Read<'a, Config>,

	pub plane: ReadStorage<'a, Plane>,
	pub energy: ReadStorage<'a, Energy>,

	pub keystate: WriteStorage<'a, KeyState>,
	pub energy_regen: WriteStorage<'a, EnergyRegen>,
}

impl<'a> System<'a> for PredatorSpecial {
	type SystemData = PredatorSpecialData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let ref info = data.config.planes[PlaneType::Predator];

		(
			&data.plane,
			&data.energy,
			&mut data.keystate,
			&mut data.energy_regen
		).join()
			.filter(|(plane, _, _, _)| **plane == PlaneType::Predator)
			.for_each(|(_, energy, keystate, energy_regen)| {
				if *energy == Energy::new(0.0) {
					keystate.special = false;
					*energy_regen = info.energy_regen;
				}
				else if keystate.special {
					*energy_regen = *PREDATOR_SPECIAL_REGEN;
				}
			});
	}
}

impl SystemInfo for PredatorSpecial {
	type Dependencies = (
		KeyHandler
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self{}
	}
}
