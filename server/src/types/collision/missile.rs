use fnv::FnvHashMap;

use protocol::MobType;
use types::{Distance, Mob, Position};

lazy_static! {
	pub static ref COLLIDERS: FnvHashMap<Mob, Vec<(Position, Distance)>> = {
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
