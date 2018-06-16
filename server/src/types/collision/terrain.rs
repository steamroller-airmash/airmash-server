use consts::TERRAIN;
use specs::world::*;
use types::*;

use types::collision::*;

// Buckets are configurable here
pub const BUCKETS_Y: usize = 64;
pub const BUCKETS_X: usize = BUCKETS_Y * 2;
pub const BUCKET_WIDTH: f32 = (32768.0 / ((BUCKETS_Y * 2) as f64)) as f32;
pub const BUCKET_HEIGHT: f32 = (32768.0 / (BUCKETS_Y as f64)) as f32;

#[derive(Debug)]
pub struct Terrain {
	pub buckets: Array2D<Bucket>,
}

impl Terrain {
	pub fn new<'a, I>(it: I, ents: &EntitiesRes) -> Self
	where
		I: Iterator<Item = &'a [i16; 3]>,
	{
		let mut buckets = Array2D::<Bucket>::new(BUCKETS_X, BUCKETS_Y);
		it.map(|var| HitCircle {
			pos: Position::new(Distance::new(var[0] as f32), Distance::new(var[1] as f32)),
			rad: Distance::new(var[2] as f32),
			layer: 0,
			ent: ents.entity(0),
		}).for_each(|hc| {
			for coord in intersected_buckets(hc.pos, hc.rad) {
				buckets.get_or_insert(coord).push(hc);
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
		Self {
			buckets: Array2D::new(0, 0),
		}
	}
}
