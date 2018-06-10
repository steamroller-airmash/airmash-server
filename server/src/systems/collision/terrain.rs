
use specs::world::*;
use types::*;
use consts::TERRAIN;

use systems::collision::array2d::Array2D;
use systems::collision::bucket::{Bucket, HitCircle};
use systems::collision::collision::{
	BUCKETS_Y,
	BUCKETS_X,
	intersected_buckets
};

#[derive(Debug)]
pub struct Terrain {
	pub buckets: Array2D<Bucket>
}

impl Terrain {
	pub fn new<'a, I>(it: I, ents: &EntitiesRes) -> Self 
	where I: Iterator<Item=&'a [i16; 3]>
	{
		let mut buckets = Array2D::<Bucket>::new(BUCKETS_Y, BUCKETS_X);
		it.map(|var| {
			HitCircle {
				pos: Position::new(
					Distance::new(var[0] as f32),
					Distance::new(var[1] as f32)
				),
				rad: Distance::new(var[2] as f32),
				layer: 0,
				ent: ents.entity(0)
			}
		})
		.for_each(|hc| {
			for coord in intersected_buckets(hc.pos, hc.rad) {
				buckets[coord].push(hc);
			}
		});

		Self { buckets }
	}

	pub fn from_default(ents: &EntitiesRes) -> Self {
		Self::new(TERRAIN.iter(), ents)
	}
}

impl Default for Terrain {
	fn default() -> Self {
		Self { buckets: Array2D::new(0, 0) }
	}
}

