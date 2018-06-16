use specs::prelude::*;
use specs::world::EntitiesRes;

use types::collision::*;
use types::*;

use protocol::MobType;

use component::channel::OnMissileTerrainCollision;
use component::event::MissileTerrainCollision;
use component::flag::IsMissile;

use fnv::FnvHashMap;

lazy_static! {
	static ref COLLIDERS: FnvHashMap<Mob, Vec<(Position, Distance)>> = {
		let mut map = FnvHashMap::default();

		let vals = [
			MobType::PredatorMissile,
			MobType::GoliathMissile,
			MobType::MohawkMissile,
			MobType::TornadoSingleMissile,
			MobType::TornadoTripleMissile,
			MobType::ProwlerMissile,
		];

		for val in vals.iter() {
			map.insert(*val, vec![(Position::default(), Distance::new(1.0))]);
		}

		map
	};
}

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
		).par_join()
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
						self.terrain.buckets[coord].collide(hc, &mut collisions);
					}
				}

				collisions
					.into_iter()
					.map(|x| MissileTerrainCollision(x))
					.collect::<Vec<MissileTerrainCollision>>()
			})
			.flatten()
			.collect::<Vec<MissileTerrainCollision>>();

		data.channel.iter_write(vec.into_iter());
	}
}
