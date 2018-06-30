use specs::*;
use types::*;

use component::flag::IsMissile;
use component::time::{LastFrame, ThisFrame};

pub struct MissileUpdate;

lazy_static! {
	static ref BOUNDARY_X: Distance = Distance::new(16384.0);
	static ref BOUNDARY_Y: Distance = Distance::new(8192.0);
	static ref SIZE_X: Distance = *BOUNDARY_X * 2.0;
	static ref SIZE_Y: Distance = *BOUNDARY_Y * 2.0;
}

#[derive(SystemData)]
pub struct MissileUpdateSystemData<'a> {
	pub pos: WriteStorage<'a, Position>,
	pub vel: WriteStorage<'a, Velocity>,
	pub mob: ReadStorage<'a, Mob>,
	pub flag: ReadStorage<'a, IsMissile>,
	pub thisframe: Read<'a, ThisFrame>,
	pub lastframe: Read<'a, LastFrame>,
}

impl<'a> System<'a> for MissileUpdate {
	type SystemData = (Read<'a, Config>, MissileUpdateSystemData<'a>);

	fn run(&mut self, (config, mut data): Self::SystemData) {
		let delta = Time::from(data.thisframe.0 - data.lastframe.0);

		(&mut data.pos, &mut data.vel, &data.mob, &data.flag)
			.join()
			.for_each(move |(pos, vel, mob, _)| {
				let info = config.mobs[*mob].missile.unwrap();

				let accel = info.accel;
				let speed = *vel;

				*vel += vel.normalized() * accel * delta;

				{
					let speed = vel.length();
					if speed > info.max_speed {
						*vel *= info.max_speed / speed;
					}
				}

				*pos += speed * delta + (*vel - speed) * delta * 0.5;

				if pos.x < -*BOUNDARY_X {
					pos.x += *SIZE_X
				}
				if pos.x > *BOUNDARY_X {
					pos.x -= *SIZE_X
				}
				if pos.y < -*BOUNDARY_Y {
					pos.y += *SIZE_Y
				}
				if pos.y > *BOUNDARY_Y {
					pos.y -= *SIZE_Y
				}
			});
	}
}

use super::MissileFireHandler;
use dispatch::SystemInfo;

impl SystemInfo for MissileUpdate {
	type Dependencies = MissileFireHandler;

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self {}
	}
}
