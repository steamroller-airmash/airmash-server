use fnv::FnvHashSet;
use specs::prelude::*;

use types::collision::*;
use types::systemdata::IsAlive;
use types::*;

use component::channel::*;
use component::event::PlayerMissileCollision;
use component::flag::*;

use consts::config::PLANE_HIT_CIRCLES;

pub struct PlayerMissileCollisionSystem;

#[derive(SystemData)]
pub struct PlayerMissileCollisionSystemData<'a> {
	pub channel: Write<'a, OnPlayerMissileCollision>,
	pub ent: Entities<'a>,

	pub pos: ReadStorage<'a, Position>,
	pub rot: ReadStorage<'a, Rotation>,
	pub team: ReadStorage<'a, Team>,
	pub plane: ReadStorage<'a, Plane>,
	pub player_flag: ReadStorage<'a, IsPlayer>,
	pub isalive: IsAlive<'a>,

	pub mob: ReadStorage<'a, Mob>,
	pub missile_flag: ReadStorage<'a, IsMissile>,
}

impl PlayerMissileCollisionSystem {
	pub fn new() -> Self {
		Self {}
	}
}

impl<'a> System<'a> for PlayerMissileCollisionSystem {
	type SystemData = PlayerMissileCollisionSystemData<'a>;

	fn run(&mut self, data: Self::SystemData) {
		let Self::SystemData {
			mut channel,
			ent,

			pos,
			rot,
			team,
			plane,
			player_flag,
			isalive,

			mob,
			missile_flag,
		} = data;

		let mut buckets = Array2D::<Bucket>::new(BUCKETS_X, BUCKETS_Y);

		(
			&*ent,
			&pos,
			&rot,
			&team,
			&plane,
			&player_flag,
			isalive.mask(),
		)
			.join()
			.for_each(|(ent, pos, rot, team, plane, ..)| {
				PLANE_HIT_CIRCLES[plane].iter().for_each(|hc| {
					let offset = hc.offset.rotate(*rot);

					let circle = HitCircle {
						pos: *pos + offset,
						rad: hc.radius,
						layer: team.0,
						ent: ent,
					};

					for coord in intersected_buckets(circle.pos, circle.rad) {
						buckets.get_or_insert(coord).push(circle);
					}
				});
			});

		let collisions = (&*ent, &pos, &team, &mob, &missile_flag)
			.par_join()
			.map(|(ent, pos, team, mob, _)| {
				let mut collisions = vec![];

				for (offset, rad) in COLLIDERS[mob].iter() {
					let hc = HitCircle {
						pos: *pos + *offset,
						rad: *rad,
						layer: team.0,
						ent: ent,
					};

					for coord in intersected_buckets(hc.pos, hc.rad) {
						match buckets.get(coord) {
							Some(bucket) => bucket.collide(hc, &mut collisions),
							None => (),
						}
					}
				}

				collisions
			}).flatten()
			.map(|x| PlayerMissileCollision(x))
			.collect::<FnvHashSet<PlayerMissileCollision>>();

		channel.iter_write(collisions.into_iter());
	}
}

use dispatch::SystemInfo;
use systems::PositionUpdate;

impl SystemInfo for PlayerMissileCollisionSystem {
	type Dependencies = PositionUpdate;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::new()
	}
}
