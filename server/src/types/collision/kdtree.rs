use super::{Collision, HitCircle};
use crate::{Distance, Position, Vector2};

use hashbrown::HashSet;
use specs::Entity;

use crate::utils::KdTree as KdTreeDs;

#[derive(Debug, Clone, Default)]
pub struct KdTree {
	tree: KdTreeDs<HitCircle>,
}

impl KdTree {
	pub fn new(circles: Vec<HitCircle>) -> Self {
		Self {
			tree: KdTreeDs::new(circles, &pos_accessor),
		}
	}

	pub fn rebuild_from<I>(&mut self, it: I)
	where
		I: IntoIterator<Item = HitCircle>,
	{
		let circles = it.into_iter().collect();
		self.tree.rebuild_from(circles, &pos_accessor);
	}

	pub fn collide<I>(&self, b: I) -> Vec<Collision>
	where
		I: Iterator<Item = HitCircle>,
	{
		let mut result = vec![];
		self.collide_nocopy(b, &mut result);
		result
	}

	pub fn collide_nocopy<I>(&self, b: I, out: &mut Vec<Collision>)
	where
		I: Iterator<Item = HitCircle>,
	{
		let mut result = vec![];
		for hc in b {
			let (pos, rad) = pos_accessor(&hc);

			self.tree.lookup(pos, rad, &mut result);

			for x in &result {
				if x.layer != hc.layer {
					out.push(Collision(hc, **x))
				};
			}
			result.clear();
		}
	}

	pub fn rough_collide(&self, hc: HitCircle) -> HashSet<Entity> {
		let (pos, rad) = pos_accessor(&hc);
		let mut result = vec![];

		self.tree.lookup(pos, rad, &mut result);

		result
			.into_iter()
			.filter(|x| x.layer != hc.layer)
			.map(|x| x.ent)
			.collect()
	}

	pub fn does_collide(&self, pos: Position, rad: Distance) -> bool {
		self.tree
			.contains_any(Vector2::new(pos.x.inner(), pos.y.inner()), rad.inner())
	}
}

fn pos_accessor(hc: &HitCircle) -> (Vector2<f32>, f32) {
	(
		Vector2::new(hc.pos.x.inner(), hc.pos.y.inner()),
		hc.rad.inner(),
	)
}
