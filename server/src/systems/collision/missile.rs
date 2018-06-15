use shrev::*;
use specs::*;
use types::*;
use types::collision::*;

use protocol::MobType;

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
			MobType::ProwlerMissile
		];

		for val in vals.iter() {
			map.insert(*val, vec![(
				Position::default(),
				Distance::new(1.0)
			)]);
		}

		map
	};
}

pub struct MissileTerrainCollisionSystem {
	terrain: Terrain
}

#[derive(SystemData)]
pub struct MissileTerrainCollisionSystemData<'a> {
	pub entities: Entities<'a>,
}
