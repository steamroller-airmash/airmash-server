use crate::Vector2;
use std::mem::MaybeUninit;
use std::ops::RangeInclusive;
use std::panic;

#[derive(Clone, Debug)]
enum KdTreeNode<T> {
	Data {
		point: Vector2<f32>,
		radius: f32,
		value: T,
	},
	Children {
		split: f32,
		/// Values greater than the split
		left: usize,
		/// Values smaller than the split
		right: usize,
	},
	Invalid,
}

enum Dimension {
	X,
	Y,
}

#[derive(Clone, Debug)]
struct AABB {
	pub x: RangeInclusive<f32>,
	pub y: RangeInclusive<f32>,
}

struct LookupResult<'a, T> {
	pos: Vector2<f32>,
	rad: f32,
	val: &'a T,
}

#[derive(Debug, Clone)]
pub struct KdTree<T> {
	// Invariant:
	//  - Root node is always at index 0
	nodes: Vec<KdTreeNode<T>>,
	max_radius: f32,
	bounds: AABB,
}

impl<T> KdTree<T> {
	pub fn is_empty(&self) -> bool {
		self.nodes.is_empty()
	}

	pub fn contains_any(&self, point: Vector2<f32>, radius: f32) -> bool {
		let mut result = vec![];
		self.lookup_rough(point, radius, &mut result);
		!result.is_empty()
	}

	fn lookup_rough<'a>(
		&'a self,
		point: Vector2<f32>,
		radius: f32,
		out: &mut Vec<LookupResult<'a, T>>,
	) {
		if self.is_empty() {
			return;
		}

		let query_rad = radius + self.max_radius;
		let mut query = AABB {
			x: (point.x - query_rad)..=(point.x + query_rad),
			y: (point.y - query_rad)..=(point.y + query_rad),
		};
		query.clip(&self.bounds);

		self.lookup_impl(0, 0, &query, out);
	}

	pub fn lookup<'a>(&'a self, point: Vector2<f32>, radius: f32, out: &mut Vec<&'a T>) {
		let mut results = vec![];
		self.lookup_rough(point, radius, &mut results);

		out.clear();
		out.extend(
			results
				.into_iter()
				.filter(|x| {
					let expect_dist = x.rad + radius;
					let dist2 = (x.pos - point).length2();

					dist2 <= expect_dist * expect_dist
				})
				.map(|x| x.val),
		);
	}

	fn lookup_impl<'a>(
		&'a self,
		index: usize,
		level: usize,
		query: &AABB,
		out: &mut Vec<LookupResult<'a, T>>,
	) {
		let ref node = self.nodes[index];

		match node {
			&KdTreeNode::Children { split, right, left } => {
				assert_ne!(right, index);
				assert_ne!(left, index);
				assert!(level < self.nodes.len());

				let (min, max) = match level_direction(level) {
					Dimension::X => (*query.x.start(), *query.x.end()),
					Dimension::Y => (*query.y.start(), *query.y.end()),
				};

				if split <= max {
					self.lookup_impl(right, level + 1, query, out);
				}
				if split >= min {
					self.lookup_impl(left, level + 1, query, out);
				}
			}
			&KdTreeNode::Data {
				point,
				radius,
				ref value,
			} if query.contains(point) => {
				out.push(LookupResult {
					pos: point,
					rad: radius,
					val: value,
				});
			}
			&KdTreeNode::Data { .. } => (),
			&KdTreeNode::Invalid => unreachable!(),
		}
	}
}

impl<T> KdTree<T> {
	fn build_nodes<F>(
		points: &mut [MaybeUninit<T>],
		level: usize,
		func: &F,
		nodes: &mut Vec<KdTreeNode<T>>,
	) -> usize
	where
		F: Fn(&T) -> (Vector2<f32>, f32),
	{
		assert_ne!(points.len(), 0);

		let direction = level_direction(level);
		let index = nodes.len();
		let get_coord = |x: &T| {
			let (pos, __) = func(x);

			match direction {
				Dimension::X => pos.x,
				Dimension::Y => pos.y,
			}
		};
		let comp_fn = |a: &MaybeUninit<T>, b: &MaybeUninit<T>| {
			get_coord(unsafe { &*a.as_ptr() })
				.partial_cmp(&get_coord(unsafe { &*b.as_ptr() }))
				.expect("Tried to insert NaN into a kd-tree")
		};

		if points.len() == 1 {
			let (pos, rad) = func(unsafe { &*points[0].as_ptr() });
			nodes.push(KdTreeNode::Data {
				point: pos,
				radius: rad,
				value: unsafe {
					std::mem::replace(&mut points[0], MaybeUninit::uninit()).assume_init()
				},
			});
			return index;
		}

		nodes.push(KdTreeNode::Invalid);

		let median_index = points.len() / 2;
		pdqselect::select_by(points, median_index, comp_fn);

		let split = get_coord(unsafe { &*points[median_index].as_ptr() });
		let (left_points, right_points) = points.split_at_mut(median_index);

		let left = Self::build_nodes(left_points, level + 1, func, nodes);
		let right = Self::build_nodes(right_points, level + 1, func, nodes);

		nodes[index] = KdTreeNode::Children { split, left, right };

		index
	}

	fn new_inner_impl<F>(
		points: &mut [MaybeUninit<T>],
		func: &F,
		mut nodes: Vec<KdTreeNode<T>>,
	) -> Self
	where
		F: Fn(&T) -> (Vector2<f32>, f32),
	{
		nodes.clear();

		if points.is_empty() {
			return Self::default();
		}

		let (pos, rad) = func(unsafe { &*points[0].as_ptr() });
		let mut bounds = AABB {
			x: pos.x..=pos.x,
			y: pos.y..=pos.y,
		};
		let mut max_radius = rad;

		for val in points.iter() {
			let (pos, rad) = func(unsafe { &*val.as_ptr() });

			bounds.expand(pos);
			max_radius = max_radius.max(rad);
		}

		Self::build_nodes(points, 0, func, &mut nodes);

		Self {
			nodes,
			max_radius,
			bounds,
		}
	}

	fn new_inner<F>(mut points: Vec<T>, func: &F, nodes: Vec<KdTreeNode<T>>) -> Self
	where
		F: panic::RefUnwindSafe + Fn(&T) -> (Vector2<f32>, f32),
	{
		let inner = || {
			use std::slice::from_raw_parts_mut;
			let slice = &mut points[..];
			let points = unsafe { from_raw_parts_mut(slice.as_ptr() as *mut _, slice.len()) };
			Self::new_inner_impl(&mut points[..], func, nodes)
		};

		let res = panic::catch_unwind(panic::AssertUnwindSafe(inner));

		unsafe { points.set_len(0) };

		match res {
			Ok(new) => new,
			Err(payload) => panic::resume_unwind(payload),
		}
	}

	/// Build a kd-tree from a sequence of points and
	/// a function which returns a pair `(point, radius)`
	pub fn new<F>(points: Vec<T>, func: &F) -> Self
	where
		F: panic::RefUnwindSafe + Fn(&T) -> (Vector2<f32>, f32),
	{
		Self::new_inner(points, func, vec![])
	}

	/// Rebuild the kd-tree with new set of points while reusing
	/// the existing storage space.
	pub fn rebuild_from<F>(&mut self, points: Vec<T>, func: &F)
	where
		F: panic::RefUnwindSafe + Fn(&T) -> (Vector2<f32>, f32),
	{
		*self = Self::new_inner(points, func, std::mem::replace(&mut self.nodes, vec![]));
	}
}

impl AABB {
	pub fn contains(&self, point: Vector2<f32>) -> bool {
		return point.x >= *self.x.start()
			&& point.x <= *self.x.end()
			&& point.y >= *self.y.start()
			&& point.y <= *self.y.end();
	}

	pub fn clip(&mut self, range: &Self) {
		let min_x = self.x.start().max(*range.x.start());
		let max_x = self.x.end().min(*range.x.end());
		let min_y = self.y.start().max(*range.y.start());
		let max_y = self.y.end().min(*range.y.end());

		*self = Self {
			x: min_x..=max_x,
			y: min_y..=max_y,
		}
	}

	// Grow the range to include the new point
	pub fn expand(&mut self, point: Vector2<f32>) {
		self.x = self.x.start().min(point.x)..=self.x.end().max(point.x);
		self.y = self.y.start().min(point.y)..=self.y.end().max(point.y);
	}

	pub fn empty() -> Self {
		Self {
			x: 0.0..=0.0,
			y: 0.0..=0.0,
		}
	}
}

impl Default for AABB {
	fn default() -> Self {
		Self {
			x: 0.0..=0.0,
			y: 0.0..=0.0,
		}
	}
}

impl<T> Default for KdTree<T> {
	fn default() -> Self {
		Self {
			nodes: vec![],
			max_radius: 0.0,
			bounds: AABB::empty(),
		}
	}
}

fn level_direction(level: usize) -> Dimension {
	match level % 2 {
		0 => Dimension::X,
		1 => Dimension::Y,
		_ => unreachable!(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty_tree_has_no_collisions() {
		let tree: KdTree<()> = KdTree::default();
		assert!(!tree.contains_any(Vector2::default(), 1000.0));
	}

	#[test]
	fn points_in_same_location() {
		let arr = vec![(Vector2::default(), 10.0), (Vector2::default(), 10.0)];
		let func = |x: &(Vector2<f32>, f32)| x.clone();

		let tree = KdTree::new(arr, &func);

		let mut result = vec![];
		tree.lookup(Vector2::new(1.0, 1.0), 0.5, &mut result);

		assert!(result.len() == 2);
	}
}
