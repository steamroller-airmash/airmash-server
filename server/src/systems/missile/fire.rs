use specs::prelude::*;
use types::systemdata::*;
use types::*;

use component::time::*;

pub struct MissileFireHandler;

#[derive(SystemData)]
pub struct MissileFireHandlerData<'a> {
	pub fire_missile: FireMissiles<'a>,
	pub plane: ReadStorage<'a, Plane>,
	pub keystate: ReadStorage<'a, KeyState>,
	pub energy: WriteStorage<'a, Energy>,
	pub config: Read<'a, Config>,
	pub this_frame: Read<'a, ThisFrame>,
	pub lastshot: ReadStorage<'a, LastShotTime>,
}

impl<'a> System<'a> for MissileFireHandler {
	type SystemData = MissileFireHandlerData<'a>;

	fn run(&mut self, mut data: Self::SystemData) {
		let this_frame = *data.this_frame;
		let config = data.config;

		let missiles = (
			&*data.fire_missile.entities,
			&data.plane,
			&data.keystate,
			&mut data.energy,
			&data.lastshot,
			data.fire_missile.is_alive.mask(),
		)
			.join()
			.filter(|(_, _, keystate, ..)| keystate.fire)
			.filter_map(|(ent, plane, _, energy, lastshot, ..)| {
				let ref info = config.planes[*plane];

				if this_frame.0 - lastshot.0 > info.fire_delay {
					Some((ent, info, energy))
				} else {
					None
				}
			}).filter(|(_, info, energy)| **energy > info.fire_energy)
			.map(|(ent, info, energy)| {
				*energy -= info.fire_energy;

				(
					ent,
					MissileFireInfo {
						pos_offset: Position::new(Distance::default(), info.missile_offset),
						rot_offset: Rotation::default(),
						ty: info.missile_type,
					},
				)
			}).collect::<Vec<_>>();

		for (ent, fire_info) in missiles {
			data.fire_missile.fire_missiles(ent, &[fire_info]);
		}
	}
}

use dispatch::SystemInfo;
use systems::PositionUpdate;

impl SystemInfo for MissileFireHandler {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
