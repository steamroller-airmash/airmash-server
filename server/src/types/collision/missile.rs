use fnv::FnvHashMap;

use protocol::MobType;
use types::{Distance, Mob, Position};

use consts::config::POWERUP_RADIUS;

lazy_static! {
	pub static ref COLLIDERS: FnvHashMap<Mob, Vec<(Position, Distance)>> = {
		let mut map = FnvHashMap::default();

		let missiles = [
			MobType::PredatorMissile,
			MobType::GoliathMissile,
			MobType::MohawkMissile,
			MobType::TornadoSingleMissile,
			MobType::TornadoTripleMissile,
			MobType::ProwlerMissile,
		];

		let powerups = [
			MobType::Upgrade,
			MobType::Shield,
			MobType::Inferno,
		];

		for val in missiles.into_iter() {
			map.insert(*val, vec![(Position::default(), Distance::new(1.0))]);
		}
		for val in powerups.into_iter() {
			map.insert(*val, vec![(Position::default(), POWERUP_RADIUS)]);
		}

		map
	};
}
