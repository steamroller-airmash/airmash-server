
use types::*;
use protocol::PlaneType;

use fnv::FnvHashMap;
use std::time::Duration;

lazy_static! {
	pub static ref FLAG_RADIUS: FnvHashMap<Plane, Distance> = {
		let mut map = FnvHashMap::default();

		// These are just random guesses
		// TODO: rev-eng these from official server
		map.insert(
			PlaneType::Predator,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Goliath,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Tornado,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Prowler,
			Distance::new(100.0)
		);

		map.insert(
			PlaneType::Mohawk,
			Distance::new(100.0)
		);

		map
	};

	pub static ref FLAG_POS: FnvHashMap<Team, Position> = {
		let mut map = FnvHashMap::default();
	
		// Blue team
		map.insert(Team(1), Position::new(Distance::new(1000.0), Distance::new(0.0)));
		// Red team
		map.insert(Team(2), Position::new(Distance::new(-1000.0), Distance::new(0.0)));

		map
	};

	pub static ref FLAG_NO_REGRAB_TIME: Duration = Duration::from_secs(5);
}
