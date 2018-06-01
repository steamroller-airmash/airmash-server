
use specs::*;
use shrev::*;
use types::*;
use fnv::FnvHashMap;

lazy_static! {
	pub static ref TERRAIN_BUCKETS: Vec<Bucket> = unimplemented!();
}

type LayerType = u16;

#[derive(Copy, Clone, Debug)]
pub struct HitCircle {
	pub pos: Position,
	pub r:   Distance,
	pub layer: u32
}

#[derive(Copy, Clone, Debug)]
pub struct CollisionEvent {
	pub e1: (Entity, HitCircle),
	pub e2: (Entity, HitCircle)
}

pub struct CollisionSystem {
	pub buckets: (usize, usize),
	pub terrain: Vec<HitCircle>
}

#[derive(Clone, Debug)]
struct Layer {
	pub entities: Vec<(Entity, HitCircle)>
}

#[derive(Clone, Debug)]
struct Bucket {
	pub layers: FnvHashMap<u16, Layer>
}

impl<'a> System<'a> for CollisionSystem {
	
}
