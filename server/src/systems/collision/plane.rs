//! This module contains a system to

use shrev::*;
use specs::prelude::*;
use specs::world::EntitiesRes;

use types::*;
use types::collision::*;

#[derive(Default)]
pub struct PlaneCollisionSystem {
	terrain: Terrain,
}

#[derive(SystemData)]
pub struct PlaneCollisionSystemData<'a> {
	pub entities: Entities<'a>,
	pub collisions: Write<'a, EventChannel<Collision>>,
	pub config: Read<'a, Config>,
	pub pos: ReadStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub planes: ReadStorage<'a, Plane>,
	pub teams: ReadStorage<'a, Team>,
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
		res.insert::<EventChannel<Collision>>(EventChannel::with_capacity(1000));
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let vec = (
			&*data.entities,
			&data.pos,
			&data.rot,
			&data.planes,
			&data.teams,
		).par_join()
			.map(|(ent, pos, rot, plane, team)| {
				let ref cfg = data.config.planes[*plane];

				let mut collisions = vec![];

				cfg.hit_circles.iter().for_each(|hc| {
					let offset = hc.offset.rotate(*rot);

					let circle = HitCircle {
						pos: *pos + offset,
						rad: hc.radius,
						layer: team.0,
						ent: ent,
					};

					for coord in intersected_buckets(*pos + offset, hc.radius) {
						trace!(target: "server", "Added to bucket {:?}", coord);
						self.terrain.buckets[coord].collide(circle, &mut collisions);
					}
				});

				collisions
			})
			.flatten()
			.collect::<Vec<Collision>>();

		data.collisions.iter_write(vec.into_iter());
	}
}
