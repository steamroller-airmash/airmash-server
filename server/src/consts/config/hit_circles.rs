use fnv::FnvHashMap;

use {Distance, Plane, Position};

#[derive(Copy, Clone, Debug, Default)]
pub struct HitCircle {
	pub radius: Distance,
	pub offset: Position,
}

impl HitCircle {
	pub fn new(x: i16, y: i16, r: i16) -> Self {
		Self {
			offset: Position::new(Distance::new(x as f32), Distance::new(y as f32)),
			radius: Distance::new(r as f32),
		}
	}
}

fn hit_circle(x: i16, y: i16, r: i16) -> HitCircle {
	HitCircle::new(x, y, r)
}

lazy_static! {
	pub static ref PLANE_HIT_CIRCLES: FnvHashMap<Plane, Vec<HitCircle>> = {
		use Plane::*;

		let mut map = FnvHashMap::default();

		map.insert(
			Predator,
			vec![
				hit_circle(0, 5, 23),
				hit_circle(0, -15, 15),
				hit_circle(0, -25, 12),
			],
		);

		map.insert(
			Goliath,
			vec![
				hit_circle(0, 0, 35),
				hit_circle(50, 14, 16),
				hit_circle(74, 26, 14),
				hit_circle(30, 8, 23),
				hit_circle(63, 22, 15),
				hit_circle(-50, 14, 16),
				hit_circle(-74, 26, 14),
				hit_circle(-30, 8, 23),
				hit_circle(-63, 22, 15),
			],
		);

		map.insert(
			Mohawk,
			vec![
				hit_circle(0, -12, 15),
				hit_circle(0, 0, 17),
				hit_circle(0, 13, 15),
				hit_circle(0, 26, 15),
			],
		);

		map.insert(
			Tornado,
			vec![
				hit_circle(0, 8, 18),
				hit_circle(14, 12, 13),
				hit_circle(-14, 12, 13),
				hit_circle(0, -12, 16),
				hit_circle(0, -26, 14),
				hit_circle(0, -35, 12),
			],
		);

		map.insert(
			Prowler,
			vec![
				hit_circle(0, 11, 25),
				hit_circle(0, -8, 18),
				hit_circle(19, 20, 10),
				hit_circle(-19, 20, 10),
				hit_circle(0, -20, 14),
			],
		);

		map
	};
}
