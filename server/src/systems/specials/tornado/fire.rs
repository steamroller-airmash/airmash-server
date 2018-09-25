use specs::prelude::*;
use types::systemdata::*;
use types::*;

use systems::handlers::packet::KeyHandler;
use systems::specials::config::*;
use systems::EnergyRegenSystem;

pub struct Fire;

#[derive(SystemData)]
pub struct FireData<'a> {
	pub fire_missile: FireMissiles<'a>,
	pub plane: ReadStorage<'a, Plane>,
	pub keystate: ReadStorage<'a, KeyState>,
	pub energy: WriteStorage<'a, Energy>,
}

impl<'a> System<'a> for Fire {
	type SystemData = FireData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let missiles = (
			&*data.fire_missile.entities,
			&data.keystate,
			&mut data.energy,
			&data.plane,
			data.fire_missile.is_alive.mask(),
		)
			.join()
			.filter(|(_, _, _, plane, ..)| **plane == Plane::Tornado)
			.filter_map(|(ent, keystate, energy, ..)| {
				if keystate.special {
					Some((ent, energy))
				} else {
					None
				}
			}).filter(|(_, energy)| **energy > *TORNADO_SPECIAL_ENERGY)
			.map(|(ent, energy)| {
				*energy -= *TORNADO_SPECIAL_ENERGY;

				(ent, &*TORNADO_MISSILE_DETAILS)
			}).collect::<Vec<_>>();

		for (ent, fire_info) in missiles {
			data.fire_missile.fire_missiles(ent, fire_info);
		}
	}
}

use dispatch::SystemInfo;
use systems::PositionUpdate;

impl SystemInfo for Fire {
	type Dependencies = (PositionUpdate, EnergyRegenSystem, KeyHandler);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
