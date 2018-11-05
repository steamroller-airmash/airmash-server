
use types::collision::{HitCircle, Collision};
use hashbrown::HashMap;
use std::cmp::Ordering;

const BUCKETS_X: u32 = 512;
const BUCKETS_Y: u32 = 512;
const BOUND_X: f32 = 16384.0;
const BOUND_Y: f32 = 8192.0;
const BUCKET_X: f32 = (32768 / 512) as f32;
const BUCKET_Y: f32 = (16384 / 512) as f32;
const INV_BX: f32 = 1.0 / BUCKET_X;
const INV_BY: f32 = 1.0 / BUCKET_Y;

#[derive(Clone, Default, Debug)]
pub struct Grid {
	circles: Vec<HitCircle>,
	buckets: HashMap<(u32, u32), (u32, u32)>,
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
	pub fn new(mut circles: Vec<HitCircle>) -> Self {
		circles.sort_by(spatial_sort);

		let mut buckets = HashMap::default();

		let mut current = (0, 0);
		let mut i = 0;
		let mut bucket_start = 0;
		for hc in &circles {
			let b = bucket(hc);

			if b != current {
				buckets.insert(current, (bucket_start, i));
				bucket_start = i;
				current = b;
			}

			i += 1;
		}

		Self { circles, buckets }
	}

	pub fn collide<I>(&self, b: I, out: &mut Vec<Collision>) 
	where
		I: Iterator<Item = HitCircle>
	{
		for hc in b {
			let b = bucket(&hc);

			let range = (hc.rad.inner() * INV_BX) as u32;
			let range_x = (
				if range > b.0 { 0 } else { b.0 - range },
				(range + b.0).min(BUCKETS_X)
			);
			let range_y = (
				if range > b.1 { 0 } else { b.1 - range },
				(range + b.1).min(BUCKETS_Y)
			);

			for x in range_x.0 .. range_x.1 {
				for y in range_y.0 .. range_y.1 {
					if let Some(&(start, idx)) = self.buckets.get(&(x, y)) {
						for i in start..idx {
							let hc2 = self.circles[i as usize];

							let dx = hc2.pos.x - hc.pos.x;
							let dy = hc2.pos.y - hc.pos.y;
							let r = hc2.rad + hc.rad;

							if dx * dx + dy * dy < r * r && hc2.layer != hc.layer {
								out.push(Collision(hc, hc2));
							}
						}
					}
				}
			}
		}
	}
}
