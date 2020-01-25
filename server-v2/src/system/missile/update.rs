use crate::component::flag::IsMissile;
use crate::ecs::prelude::*;
use crate::resource::{Config, CurrentFrame, LastFrame};
use crate::{Distance, Mob, Position, Time, Velocity};

const BOUNDARY_X: Distance = Distance::new(16384.0);
const BOUNDARY_Y: Distance = Distance::new(8192.0);
const SIZE_X: Distance = Distance::new(BOUNDARY_X.value * 2.0);
const SIZE_Y: Distance = Distance::new(BOUNDARY_Y.value * 2.0);

/// Every frame, update the missile positions.
#[system]
fn missile_update<'a>(
    entities: Entities<'a>,
    mut pos: WriteStorage<'a, Position>,
    mut vel: WriteStorage<'a, Velocity>,
    mob: ReadStorage<'a, Mob>,
    flag: ReadStorage<'a, IsMissile>,

    config: Read<'a, Config>,
    this_frame: ReadExpect<'a, CurrentFrame>,
    last_frame: ReadExpect<'a, LastFrame>,
) {
    let delta = Time::from(this_frame.0 - last_frame.0);

    for (pos, vel, mob, ..) in (&mut pos, &mut vel, &mob, &flag, &entities).join() {
        let info = config.mobs[*mob].missile.as_ref().unwrap();

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

        if pos.x < -BOUNDARY_X {
            pos.x += SIZE_X;
        } else if pos.x > BOUNDARY_X {
            pos.x -= SIZE_X;
        }

        if pos.y < -BOUNDARY_Y {
            pos.y += SIZE_Y;
        } else if pos.y > BOUNDARY_Y {
            pos.y -= SIZE_Y;
        }
    }
}
