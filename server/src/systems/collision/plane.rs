use specs::prelude::*;
use specs::world::EntitiesRes;

use crate::types::collision::*;
use crate::types::*;

use crate::component::channel::*;
use crate::component::event::PlayerTerrainCollision;

use crate::consts::config::PLANE_HIT_CIRCLES;

#[derive(Default)]
pub struct PlaneCollisionSystem {
	terrain: Terrain,
}

#[derive(SystemData)]
pub struct PlaneCollisionSystemData<'a> {
	entities: Entities<'a>,
	collisions: Write<'a, OnPlayerTerrainCollision>,
	pos: ReadStorage<'a, Position>,
	rot: ReadStorage<'a, Rotation>,
	planes: ReadStorage<'a, Plane>,
	teams: ReadStorage<'a, Team>,
}

impl PlaneCollisionSystem {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<'a> System<'a> for PlaneCollisionSystem {
	type SystemData = PlaneCollisionSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.terrain = Terrain::from_default(&*res.fetch::<EntitiesRes>());

		// Hopefully 1000 collision events is enough during
		// each 16ms frame. If not, this number should be
		// increased.
		res.insert::<OnPlayerTerrainCollision>(OnPlayerTerrainCollision::with_capacity(1000));
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let vec = (
			&*data.entities,
			&data.pos,
			&data.rot,
			&data.planes,
			&data.teams,
		)
			.join()
			.map(|(ent, pos, rot, plane, team)| {
				let it = (*PLANE_HIT_CIRCLES)[plane].iter().map(|hc| {
					let offset = hc.offset.rotate(*rot);

					HitCircle {
						pos: *pos + offset,
						rad: hc.radius,
						layer: team.0,
						ent: ent,
					}
				});

				self.terrain
					.collide(it)
					.into_iter()
					.map(|x| PlayerTerrainCollision(x))
					.collect::<Vec<PlayerTerrainCollision>>()
			})
			.flatten()
			.collect::<Vec<PlayerTerrainCollision>>();

		data.collisions.iter_write(vec.into_iter());
	}
}

system_info! {
	impl SystemInfo for PlaneCollisionSystem {
		type Dependencies = crate::systems::PositionUpdate;
	}
}
