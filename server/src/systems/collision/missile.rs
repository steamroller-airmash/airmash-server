use specs::prelude::*;
use specs::world::EntitiesRes;

use types::collision::*;
use types::*;

use component::channel::OnMissileTerrainCollision;
use component::event::MissileTerrainCollision;
use component::flag::IsMissile;

#[derive(Default)]
pub struct MissileTerrainCollisionSystem {
	terrain: Terrain,
}

#[derive(SystemData)]
pub struct MissileTerrainCollisionSystemData<'a> {
	pub entities: Entities<'a>,
	pub channel: Write<'a, OnMissileTerrainCollision>,

	pub pos: ReadStorage<'a, Position>,
	pub mob: ReadStorage<'a, Mob>,
	pub team: ReadStorage<'a, Team>,
	pub flag: ReadStorage<'a, IsMissile>,
}

impl MissileTerrainCollisionSystem {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<'a> System<'a> for MissileTerrainCollisionSystem {
	type SystemData = MissileTerrainCollisionSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.terrain = Terrain::from_default(&*res.fetch::<EntitiesRes>());

		// Hopefully 1000 collision events is enough during
		// each 16ms frame. If not, this number should be
		// increased.
		res.insert::<OnMissileTerrainCollision>(OnMissileTerrainCollision::with_capacity(1000));
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let vec = (
			&*data.entities,
			&data.pos,
			&data.mob,
			&data.team,
			&data.flag,
		)
			.par_join()
			.map(|(ent, pos, mob, team, _)| {
				let mut collisions = vec![];

				for (offset, rad) in COLLIDERS[mob].iter() {
					let hc = HitCircle {
						pos: *pos + *offset,
						rad: *rad,
						layer: team.0,
						ent: ent,
					};

					for coord in intersected_buckets(hc.pos, hc.rad) {
						match self.terrain.buckets.get(coord) {
							Some(bucket) => bucket.collide(hc, &mut collisions),
							None => (),
						}
					}
				}

				collisions
					.into_iter()
					.map(|x| MissileTerrainCollision(x))
					.collect::<Vec<MissileTerrainCollision>>()
			}).flatten()
			.collect::<Vec<MissileTerrainCollision>>();

		data.channel.iter_write(vec.into_iter());
	}
}

use dispatch::SystemInfo;
use systems::PositionUpdate;

impl SystemInfo for MissileTerrainCollisionSystem {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}
