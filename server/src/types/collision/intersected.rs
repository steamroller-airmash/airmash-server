use types::collision::*;
use types::{Distance, Position};

/// TODO: Replace this with something that doesn't
/// need to allocate (a generator most likely).
/// Note: generators are still a nightly-only feature
pub fn intersected_buckets(pos: Position, rad: Distance) -> impl Iterator<Item = (usize, usize)> {
	let mut vals = vec![];

	const ADJUST_Y: f32 = (BUCKETS_Y / 2) as f32 + 0.5;
	const ADJUST_X: f32 = (BUCKETS_X / 2) as f32 + 0.5;

	let y_max = (((pos.y + rad).inner() / BUCKET_HEIGHT).ceil() + ADJUST_Y) as isize;
	let y_min = (((pos.y - rad).inner() / BUCKET_HEIGHT).floor() + ADJUST_Y) as isize;
	let x_max = (((pos.x + rad).inner() / BUCKET_WIDTH).ceil() + ADJUST_X) as isize;
	let x_min = (((pos.x - rad).inner() / BUCKET_WIDTH).floor() + ADJUST_X) as isize;

	trace!(target: "server", "Checking HC ({:?}, {})", pos, rad);
	trace!(target: "server", "HC BB {} {} {} {}", y_max, y_min, x_max, x_min);

	for x in x_min.max(0)..x_max.min(BUCKETS_X as isize) {
		for y in y_min.max(0)..y_max.min(BUCKETS_Y as isize) {
			vals.push((x as usize, y as usize));
		}
	}

	vals.into_iter()
}
