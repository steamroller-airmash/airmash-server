use consts::TERRAIN;
use specs::world::*;
use types::*;

use types::collision::*;

// Buckets are configurable here
pub const BUCKETS_Y: usize = 64;
pub const BUCKETS_X: usize = BUCKETS_Y * 2;
pub const BUCKET_WIDTH: f32 = (32768.0 / ((BUCKETS_Y * 2) as f64)) as f32;
pub const BUCKET_HEIGHT: f32 = (32768.0 / (BUCKETS_Y as f64)) as f32;

#[derive(Default, Debug)]
pub struct Terrain {
	pub buckets: Grid,
}

impl Terrain {
	pub fn new<'a, I>(it: I, ents: &EntitiesRes) -> Self
	where
		I: Iterator<Item = &'a [i16; 3]>,
	{
		Self::with_entity(it, ents.entity(0))
	}

	pub fn with_entity<'a, I>(it: I, ent: Entity) -> Self
	where
		I: Iterator<Item = &'a [i16; 3]>,
	{
		let grid = Grid::new(
			it.map(|var| HitCircle {
				pos: Position::new(var[0] as f32, var[1] as f32),
				rad: Distance::new(var[2] as f32),
				layer: 0,
				ent: ent,
			})
			.collect(),
		);

		Self { buckets: grid }
	}

	pub fn from_default(ents: &EntitiesRes) -> Self {
		Self::new(TERRAIN.iter(), ents)
	}

	pub fn collide<I>(&self, it: I, out: &mut Vec<Collision>)
	where
		I: Iterator<Item = HitCircle>,
	{
		self.buckets.collide(it, out)
	}
}
