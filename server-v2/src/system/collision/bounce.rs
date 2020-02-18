use crate::component::KeyState;
use crate::ecs::prelude::*;
use crate::protocol::server::EventBounce;
use crate::resource::{CurrentFrame, StartTime};
use crate::sysdata::Connections;
use crate::util::ToClock;
use crate::*;

use crate::event::collision::PlayerTerrainCollision;

#[event_handler]
fn player_bounce<'a>(
    evt: &PlayerTerrainCollision,

    vel: &mut WriteStorage<'a, Velocity>,
    pos: &ReadStorage<'a, Position>,
    rot: &ReadStorage<'a, Rotation>,
    team: &ReadStorage<'a, Team>,
    plane: &ReadStorage<'a, Plane>,
    keystate: &ReadStorage<'a, KeyState>,

    current: &ReadExpect<'a, CurrentFrame>,
    start: &ReadExpect<'a, StartTime>,
    conns: &Connections<'a>,
) {
    let entity = evt.player().expect("Player had no entity");
    let relative = (evt.player.pos - evt.terrain.pos).normalized();
    let max_spd = *try_get!(entity, vel);

    let speed = relative * Speed::max(max_spd.length(), Speed::new(1.0));

    let pos = *try_get!(entity, pos);
    let rot = *try_get!(entity, rot);
    let keystate = try_get!(entity, keystate);
    let plane = *try_get!(entity, plane);
    let state = keystate.to_server(&plane);

    *try_get!(entity, mut vel) = speed;

    let packet = EventBounce {
        clock: (current.0 - start.0).to_clock() as u32,
        id: entity.into(),
        pos,
        rot,
        speed,
        keystate: state,
    };

    if keystate.stealthed {
        let team = *try_get!(entity, team);
        conns.send_to_team_visible(pos, team, packet);
    } else {
        conns.send_to_visible(pos, packet);
    }
}
