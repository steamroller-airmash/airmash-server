use specs::Entity;
use types::*;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitCircle {
	pub pos: Position,
	pub rad: Distance,
	pub layer: u16,
	pub ent: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collision(pub HitCircle, pub HitCircle);

// If hitcircles have a NaN in them we're already done for
// this lets us use itertools unique() method
impl Eq for Collision {}

impl Hash for Collision {
	fn hash<H: Hasher>(&self, h: &mut H) {
		h.write_u16(self.0.layer ^ self.1.layer);
		h.write_u32(self.0.ent.id() ^ self.1.ent.id());
		h.write_i32(self.0.ent.gen().id() ^ self.1.ent.gen().id());
	}
}

#[derive(Clone, Debug, Default)]
pub struct Bucket {
	elems: Vec<HitCircle>,
}

impl HitCircle {
	pub fn intersects(a: &HitCircle, b: &HitCircle) -> bool {
		// Compare squared distances to avoid a square root
		(a.pos - b.pos).length2() < (a.rad + b.rad) * (a.rad + b.rad)
	}
}
