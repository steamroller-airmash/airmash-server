
//! This module contains a system to 

use specs::*;
use shrev::*;

use types::*;
use fnv::FnvHashMap;

use systems::collision::array2d::Array2D;
use systems::collision::bucket::*;

// Buckets are configurable here
const BUCKETS_Y: usize = 64;
const BUCKETS_X: usize = BUCKETS_Y * 2;
const BUCKET_WIDTH: f32 = (16384.0 / ((BUCKETS_Y * 2) as f64)) as f32;
const BUCKET_HEIGHT: f32 = (16384.0 / (BUCKETS_Y as f64)) as f32;

pub struct CollisionSystem {}

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
	pub entities:   Entities<'a>,
	pub collisions: Write<'a, EventChannel<Collision>>,
	pub config:     Read<'a, Config>,
	pub pos:        ReadStorage<'a, Position>,
	pub rot:        ReadStorage<'a, Rotation>,
	pub planes:     ReadStorage<'a, Plane>,
	pub teams:      ReadStorage<'a, Team>
}

impl CollisionSystem {
	pub fn new() -> Self {
		Self { }
	}
}

/// TODO: Replace this with something that doesn't
/// need to allocate (a generator most likely).
/// Note: generators are still a nightly-only feature
fn intersected_buckets(pos: Position, rad: Distance) -> impl Iterator<Item=(usize, usize)> {
	let mut vals = vec![];

	let y_max = (((pos.y + rad).inner() / BUCKET_HEIGHT) as isize + (BUCKETS_Y / 2) as isize) as usize;
	let y_min = (((pos.y - rad).inner() / BUCKET_HEIGHT) as isize + (BUCKETS_Y / 2) as isize) as usize;
	let x_max = (((pos.x + rad).inner() / BUCKET_WIDTH) as isize + (BUCKETS_X / 2) as isize) as usize;
	let x_min = (((pos.x - rad).inner() / BUCKET_WIDTH) as isize + (BUCKETS_X / 2) as isize) as usize;

	for x in x_min..x_max {
		for y in y_min..y_max {
			vals.push((x, y));
		}
	}

	vals.into_iter()
}

impl<'a> System<'a> for CollisionSystem {
	type SystemData = CollisionSystemData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		// Hopefully 1000 collision events is enough during 
		// each 16ms frame. If not, this number should be 
		// increased.
		res.insert::<EventChannel<Collision>>(EventChannel::with_capacity(1000));
	}

	fn run(&mut self, mut data: Self::SystemData) {
		let mut buckets = Array2D::<Bucket>::new(BUCKETS_Y, BUCKETS_Y * 2);

		(&*data.entities, &data.pos, &data.rot, &data.planes, &data.teams).join()
			.for_each(|(ent, pos, rot, plane, team)| {
				let ref cfg = data.config.planes[*plane];

				for hc in cfg.hit_circles.iter() {
					let offset = hc.offset.rotate(*rot);

					let circle = HitCircle {
						pos: *pos + offset,
						rad: hc.radius,
						layer: team.0,
						ent: ent
					};
					
					for coord in intersected_buckets(*pos + offset, hc.radius){
						buckets[coord].push(circle);
					}
				}
			});

		let mut isects = vec![];
		buckets.iter().for_each(|bucket| {
			bucket.collide(&mut isects);
		});

		data.collisions.iter_write(isects.into_iter());
	}
}
