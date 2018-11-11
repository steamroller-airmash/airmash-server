use hashbrown::HashMap;
use std::cmp::Ordering;

use types::collision::{Collision, HitCircle};

const BUCKETS_X: u32 = BUCKETS_Y * 2;
const BUCKETS_Y: u32 = 512;
const BOUND_X: f32 = 16384.0;
const BOUND_Y: f32 = 8192.0;
const BUCKET_X: f32 = (32768 / 512) as f32;
const BUCKET_Y: f32 = (16384 / 512) as f32;
const INV_BX: f32 = 1.0 / BUCKET_X;
const INV_BY: f32 = 1.0 / BUCKET_Y;

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
/// # let circles_from_elsewhere = return;
/// // Hit circles from terrain, planes, etc.
/// let circles: Vec<HitCircle> = circles_from_elsewhere;
///
/// // Grid takes ownership of the hitcircle vector
/// let grid = Grid::new(circles);
/// # }
/// ```
///
/// # Notes
/// - Due to (current) bucket sizes no bucket will
///   contain more than 1 terrain hitcircle.
#[derive(Clone, Default, Debug)]
pub struct Grid {
	circles: Vec<HitCircle>,
	buckets: HashMap<(u32, u32), (u32, u32)>,
	max_r: f32,
}

fn bucket(a: &HitCircle) -> (u32, u32) {
	let x = a.pos.x.inner().min(BOUND_X).max(-BOUND_X) + BOUND_X;
	let y = a.pos.y.inner().min(BOUND_Y).max(-BOUND_Y) + BOUND_Y;

	((x * INV_BX) as u32, (y * INV_BY) as u32)
}

fn spatial_sort(a: &HitCircle, b: &HitCircle) -> Ordering {
	let bounds_a = bucket(a);
	let bounds_b = bucket(b);

	match bounds_a.1.cmp(&bounds_b.1) {
		Ordering::Equal => bounds_a.0.cmp(&bounds_b.0),
		x => x,
	}
}

impl Grid {
	/// Create a new `Grid` from a list of hit circles.
	pub fn new(mut circles: Vec<HitCircle>) -> Self {
		circles.sort_by(spatial_sort);

		let mut buckets = HashMap::default();

		let mut current = (0, 0);
		let mut i = 0;
		let mut bucket_start = 0;
		let mut max_r = 0.0.into();
		for hc in &circles {
			let b = bucket(hc);

			if hc.rad.inner() > max_r {
				max_r = hc.rad.inner();
			}

			if b != current {
				if i != bucket_start {
					buckets.insert(current, (bucket_start, i));
				}
				bucket_start = i;
				current = b;
			}

			i += 1;
		}

		Self {
			circles,
			buckets,
			max_r,
		}
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
			let b = bucket(&hc);

			// Largest radii that need to be checked in each direction.
			// If this is larger than it needs to be, then the algorithm
			// will be slower, but if it's too small then collisions that
			// are supposed to be found will be missed
			let rx = ((hc.rad.inner() + self.max_r + BUCKET_X) * INV_BX) as u32;
			let ry = ((hc.rad.inner() + self.max_r + BUCKET_Y) * INV_BY) as u32;
			let range_x = (
				if rx > b.0 { 0 } else { b.0 - rx },
				(rx + b.0 + 1).min(BUCKETS_X),
			);
			let range_y = (
				if ry > b.1 { 0 } else { b.1 - ry },
				(ry + b.1 + 1).min(BUCKETS_Y),
			);

			for x in range_x.0..range_x.1 {
				for y in range_y.0..range_y.1 {
					if let Some(&(start, len)) = self.buckets.get(&(x, y)) {
						for i in start..len {
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
		}
	}

	pub fn into_inner(self) -> Vec<HitCircle> {
		self.circles
	}

	pub fn iter(&self) -> impl Iterator<Item = &HitCircle> {
		self.circles.iter()
	}
}
