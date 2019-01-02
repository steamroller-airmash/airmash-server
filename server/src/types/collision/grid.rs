use hashbrown::HashSet;
use specs::Entity;

use std::cmp::Ordering;
use std::mem;

use types::collision::{Collision, HitCircle};
use types::{Distance, Position};

const BUCKETS_X: u32 = BUCKETS_Y * 2;
const BUCKETS_Y: u32 = 64;
const NUM_BUCKETS: u32 = BUCKETS_X * BUCKETS_Y;
const BOUND_X: f32 = 16384.0;
const BOUND_Y: f32 = 8192.0;
const BUCKET_SIZE_X: f32 = 32768.0 / (BUCKETS_X - 1) as f32;
const BUCKET_SIZE_Y: f32 = 16384.0 / (BUCKETS_Y - 1) as f32;
const INV_BX: f32 = 1.0 / BUCKET_SIZE_X;
const INV_BY: f32 = 1.0 / BUCKET_SIZE_Y;

/// Efficient spatial-index for collision checking.
///
/// This is a lookup structure to efficiently check
/// for circle-circle collisions without having `O(n^2)`
/// runtime in the average case. The way it works is
/// by putting circles into buckets, the number of
/// buckets is stored in `BUCKETS_X` and `BUCKETS_Y` in
/// the source file of this struct.
///
/// When checking a circle `x` for a collision, the
/// buckets are used as a preliminary filter to quickly
/// get a set of possible colliding circles. After that
/// they are checked linearly (`O(n)` for each circle).
///
/// # Caveats
/// - This can still be `O(n^2)` worst case if all the
///   circles are in a single bucket. (e.g. if there
///   are multiple planes and missiles in the same
///   bucket.)
/// - Constructing the `Grid` is `O(n log n)` since the
///   source array must be sorted.
///
/// # Example
/// Creating a grid from a list of circles.
/// ```
/// # extern crate airmash_server;
/// # use airmash_server::types::collision::{Grid, HitCircle};
/// # fn main() {
/// # let circles_from_elsewhere = vec![];
/// // Hit circles from terrain, planes, etc.
/// let circles: Vec<HitCircle> = circles_from_elsewhere;
///
/// // Grid takes ownership of the hitcircle vector
/// let grid = Grid::new(circles);
/// # }
/// ```
///
/// # Example
/// Recreating a grid with a new set of hit circles while
/// reusing the memory of the old one.
/// ```
/// # extern crate airmash_server;
/// # use airmash_server::types::collision::{Grid, HitCircle};
/// # fn main() {
/// # let circles_from_elsewhere = vec![];
/// // Of course normally this would be an existing
/// // grid lying around somewhere.
/// let mut grid = Grid::default();
/// // Hit circles from terrain, planes, etc.
/// // Note that this can be any iterator.
/// let circles: Vec<HitCircle> = circles_from_elsewhere;
///
/// // Rebuild the grid using the new circles
/// grid.rebuild_from(circles.into_iter());
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Grid {
	circles: Vec<HitCircle>,
	buckets: Vec<(u32, u32)>,
	max_r: f32,
}

impl Default for Grid {
	fn default() -> Self {
		Self::new(vec![])
	}
}

fn bucket(pos: Position) -> (u32, u32) {
	let x = pos.x.inner().min(BOUND_X).max(-BOUND_X) + BOUND_X;
	let y = pos.y.inner().min(BOUND_Y).max(-BOUND_Y) + BOUND_Y;

	((x * INV_BX) as u32, (y * INV_BY) as u32)
}

fn spatial_sort(a: &HitCircle, b: &HitCircle) -> Ordering {
	let bounds_a = bucket(a.pos);
	let bounds_b = bucket(b.pos);

	match bounds_a.1.cmp(&bounds_b.1) {
		Ordering::Equal => bounds_a.0.cmp(&bounds_b.0),
		x => x,
	}
}

impl Grid {
	fn new_inner(mut circles: Vec<HitCircle>, mut buckets: Vec<(u32, u32)>) -> Self {
		circles.sort_by(spatial_sort);

		let mut i: usize = 0;
		let mut max_r = 0.0;

		for y in 0..BUCKETS_Y {
			for x in 0..BUCKETS_X {
				let start = i;

				while i < circles.len() && (x, y) == bucket(circles[i].pos) {
					if circles[i].rad.inner() > max_r {
						max_r = circles[i].rad.inner();
					}

					i += 1;
				}

				buckets[(y * BUCKETS_X + x) as usize] = (start as u32, (i - start) as u32);
			}
		}

		for i in 0..buckets.len() {
			let (x, y) = buckets[i];
			if !(x != 0xFFFFFFFF && y != 0xFFFFFFFF) {
				assert!(x != 0xFFFFFFFF && y != 0xFFFFFFFF);
			}
		}

		Self {
			circles,
			buckets,
			max_r,
		}
	}
	fn create_buckets() -> Vec<(u32, u32)> {
		vec![(0xFFFFFFFF as u32, 0xFFFFFFFF as u32); NUM_BUCKETS as usize]
	}

	/// Create a new `Grid` from a list of hit circles.
	pub fn new(circles: Vec<HitCircle>) -> Self {
		Self::new_inner(circles, Self::create_buckets())
	}

	/// Build a new grid by reusing the memory of an existing one.
	pub fn rebuild_from<I>(&mut self, it: I)
	where
		I: IntoIterator<Item = HitCircle>,
	{
		self.circles.clear();
		self.circles.extend(it);

		// Only should happen if rebuilding from
		// a default-constructed grid
		if self.buckets.len() != NUM_BUCKETS as usize {
			self.buckets = Self::create_buckets()
		}

		let old = mem::replace(self, Self::default());

		*self = Self::new_inner(old.circles, old.buckets);
	}

	/// Collide a number of circles against all circles
	/// currently within the grid.
	pub fn collide<I>(&self, b: I) -> Vec<Collision>
	where
		I: Iterator<Item = HitCircle>,
	{
		let mut result = vec![];
		self.collide_nocopy(b, &mut result);
		result
	}
	/// Collide a number of circles against all circles
	/// currently within the grid.
	///
	/// # Notes
	/// Eventually the return type of this function will
	/// be replaced with a generator once generators are
	/// available on stable. This will prevent having to
	/// allocate a vec when doing collision checking.
	pub fn collide_nocopy<I>(&self, b: I, out: &mut Vec<Collision>)
	where
		I: Iterator<Item = HitCircle>,
	{
		for hc in b {
			let b = bucket(hc.pos);

			// Largest radii that need to be checked in each direction.
			// If this is larger than it needs to be, then the algorithm
			// will be slower, but if it's too small then collisions that
			// are supposed to be found will be missed
			let rx = ((hc.rad.inner() + self.max_r + BUCKET_SIZE_X) * INV_BX) as u32;
			let ry = ((hc.rad.inner() + self.max_r + BUCKET_SIZE_Y) * INV_BY) as u32;
			let range_x = (
				if rx > b.0 { 0 } else { b.0 - rx },
				(rx + b.0 + 1).min(BUCKETS_X - 1),
			);
			let range_y = (
				if ry > b.1 { 0 } else { b.1 - ry },
				(ry + b.1 + 1).min(BUCKETS_Y - 1),
			);

			for y in range_y.0..range_y.1 {
				let (start, _) = self.buckets[(y * BUCKETS_X + range_x.0) as usize];
				let (end, endlen) = self.buckets[(y * BUCKETS_X + range_x.1) as usize];
				let end = end + endlen;
				for i in start..end {
					let hc2 = self.circles[i as usize];
					let r = hc2.rad + hc.rad;
					let dist2 = (hc.pos - hc2.pos).length2();

					if dist2 < r * r && hc2.layer != hc.layer {
						out.push(Collision(hc, hc2));
					}
				}
			}
		}
	}

	/// Test whether the hit circle defined by the given
	/// position and radius collides. This method doesn't
	/// check the layer of the hit circle.
	pub fn does_collide(&self, pos: Position, rad: Distance) -> bool {
		let b = bucket(pos);

		// Largest radii that need to be checked in each direction.
		// If this is larger than it needs to be, then the algorithm
		// will be slower, but if it's too small then collisions that
		// are supposed to be found will be missed
		let rx = ((rad.inner() + self.max_r + BUCKET_SIZE_X) * INV_BX) as u32;
		let ry = ((rad.inner() + self.max_r + BUCKET_SIZE_Y) * INV_BY) as u32;
		let range_x = (
			if rx > b.0 { 0 } else { b.0 - rx },
			(rx + b.0 + 1).min(BUCKETS_X - 1),
		);
		let range_y = (
			if ry > b.1 { 0 } else { b.1 - ry },
			(ry + b.1 + 1).min(BUCKETS_Y - 1),
		);

		for y in range_y.0..range_y.1 {
			let (start, _) = self.buckets[(y * BUCKETS_X + range_x.0) as usize];
			let (end, endlen) = self.buckets[(y * BUCKETS_X + range_x.1) as usize];
			let end = end + endlen;
			for i in start..end {
				let hc = self.circles[i as usize];
				let r = hc.rad + rad;
				let dist2 = (pos - hc.pos).length2();

				if dist2 < r * r {
					return true;
				}
			}
		}

		false
	}

	/// Indicate whether a hit circle collides with
	/// any circles within the grid. (This is only
	/// an approximation.)
	pub fn test_collide(&self, hc: HitCircle) -> bool {
		let b = bucket(hc.pos);

		// Largest radii that need to be checked in each direction.
		// If this is larger than it needs to be, then the algorithm
		// will be slower, but if it's too small then collisions that
		// are supposed to be found will be missed
		let rx = ((hc.rad.inner() + self.max_r + BUCKET_SIZE_X) * INV_BX) as u32;
		let ry = ((hc.rad.inner() + self.max_r + BUCKET_SIZE_Y) * INV_BY) as u32;
		let range_x = (
			if rx > b.0 { 0 } else { b.0 - rx },
			(rx + b.0 + 1).min(BUCKETS_X - 1),
		);
		let range_y = (
			if ry > b.1 { 0 } else { b.1 - ry },
			(ry + b.1 + 1).min(BUCKETS_Y - 1),
		);

		for y in range_y.0..range_y.1 {
			let (start, _) = self.buckets[(y * BUCKETS_X + range_x.0) as usize];
			let (end, endlen) = self.buckets[(y * BUCKETS_X + range_x.1) as usize];
			let end = end + endlen;

			if start != end {
				return true;
			}
		}

		false
	}
	/// Get all entities that the hit circle could potentially
	/// collide with
	pub fn rough_collide(&self, hc: HitCircle) -> HashSet<Entity> {
		let b = bucket(hc.pos);

		// Largest radii that need to be checked in each direction.
		// If this is larger than it needs to be, then the algorithm
		// will be slower, but if it's too small then collisions that
		// are supposed to be found will be missed
		let rx = ((hc.rad.inner() + self.max_r + BUCKET_SIZE_X) * INV_BX) as u32;
		let ry = ((hc.rad.inner() + self.max_r + BUCKET_SIZE_Y) * INV_BY) as u32;
		let range_x = (
			if rx > b.0 { 0 } else { b.0 - rx },
			(rx + b.0 + 1).min(BUCKETS_X - 1),
		);
		let range_y = (
			if ry > b.1 { 0 } else { b.1 - ry },
			(ry + b.1 + 1).min(BUCKETS_Y - 1),
		);

		let mut result = HashSet::default();

		for y in range_y.0..range_y.1 {
			let (start, _) = self.buckets[(y * BUCKETS_X + range_x.0) as usize];
			let (end, endlen) = self.buckets[(y * BUCKETS_X + range_x.1) as usize];
			let end = end + endlen;

			for i in start..end {
				result.insert(self.circles[i as usize].ent);
			}
		}

		result
	}

	pub fn into_inner(self) -> Vec<HitCircle> {
		self.circles
	}

	pub fn iter(&self) -> impl Iterator<Item = &HitCircle> {
		self.circles.iter()
	}
}
