use crate::types::systemdata::*;
use crate::types::*;
use specs::prelude::*;

use crate::systems::handlers::packet::KeyHandler;
use crate::systems::specials::config::*;
use crate::systems::EnergyRegenSystem;

pub struct Fire;

#[derive(SystemData)]
pub struct FireData<'a> {
	plane: ReadStorage<'a, Plane>,
	keystate: ReadStorage<'a, KeyState>,
	powerups: ReadStorage<'a, Powerups>,

	energy: WriteStorage<'a, Energy>,

	fire_missile: FireMissiles<'a>,
	is_alive: IsAlive<'a>,
	entities: Entities<'a>,
}

impl<'a> System<'a> for Fire {
	type SystemData = FireData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let powerups = data.powerups;

		let missiles = (
			&*data.entities,
			&data.keystate,
			&mut data.energy,
			&data.plane,
			data.is_alive.mask(),
		)
			.join()
			.filter(|(_, _, _, plane, ..)| **plane == Plane::Tornado)
			.filter_map(|(ent, keystate, energy, ..)| {
				if keystate.special {
					Some((ent, energy))
				} else {
					None
				}
			})
			.filter(|(_, energy)| **energy > *TORNADO_SPECIAL_ENERGY)
			.map(|(ent, energy)| {
				*energy -= *TORNADO_SPECIAL_ENERGY;

				let inferno = match powerups.get(ent) {
					Some(powerups) => powerups.inferno(),
					None => false,
				};

				if inferno {
					(ent, &*TORNADO_INFERNO_MISSILE_DETAILS)
				} else {
					(ent, &*TORNADO_MISSILE_DETAILS)
				}
			})
			.collect::<Vec<_>>();

		for (ent, fire_info) in missiles {
			data.fire_missile.fire_missiles(ent, fire_info);
		}
	}
}

use crate::dispatch::SystemInfo;
use crate::systems::PositionUpdate;

impl SystemInfo for Fire {
	type Dependencies = (PositionUpdate, EnergyRegenSystem, KeyHandler);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
