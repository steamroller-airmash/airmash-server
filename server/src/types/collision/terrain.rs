use consts::TERRAIN;
use specs::world::*;
use types::*;

use types::collision::*;

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

	pub fn collide<I>(&self, it: I) -> Vec<Collision>
	where
		I: Iterator<Item = HitCircle>,
	{
		self.buckets.collide(it)
	}
}
