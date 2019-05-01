use specs::prelude::*;
use specs::world::EntitiesRes;

use component::channel::OnMissileTerrainCollision;
use component::event::MissileTerrainCollision;
use component::flag::IsMissile;
use systems;
use types::collision::*;
use types::*;

#[derive(Default)]
pub struct MissileTerrainCollisionSystem {
	terrain: Terrain,
}

#[derive(SystemData)]
pub struct MissileTerrainCollisionSystemData<'a> {
	entities: Entities<'a>,
	channel: Write<'a, OnMissileTerrainCollision>,

	pos: ReadStorage<'a, Position>,
	mob: ReadStorage<'a, Mob>,
	team: ReadStorage<'a, Team>,
	flag: ReadStorage<'a, IsMissile>,
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
			.join()
			.map(|(ent, pos, mob, team, _)| {
				let it = COLLIDERS[mob].iter().map(|(offset, rad)| HitCircle {
					pos: *pos + *offset,
					rad: *rad,
					layer: team.0,
					ent: ent,
				});

				self.terrain
					.collide(it)
					.into_iter()
					.map(|x| MissileTerrainCollision(x))
					.collect::<Vec<MissileTerrainCollision>>()
			})
			.flatten()
			.collect::<Vec<MissileTerrainCollision>>();

		data.channel.iter_write(vec.into_iter());
	}
}

system_info! {
	impl SystemInfo for MissileTerrainCollisionSystem {
		type Dependencies = systems::PositionUpdate;
	}
}
