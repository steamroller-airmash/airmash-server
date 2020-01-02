use crate::component::{flag::ForcePlayerUpdate, time::LastKeyTime, KeyState};
use crate::ecs::prelude::*;
use crate::protocol::{client::Key, KeyCode};
use crate::resource::{packet::ClientPacket, CurrentFrame};
use crate::sysdata::Connections;

#[event_handler]
fn handle_key<'a>(
    evt: &ClientPacket<Key>,
    conns: &Connections<'a>,
    this_frame: &ReadExpect<'a, CurrentFrame>,

    force: &mut WriteStorage<'a, ForcePlayerUpdate>,
    keystate: &mut WriteStorage<'a, KeyState>,
    last_key: &mut WriteStorage<'a, LastKeyTime>,
) {
    let player = match conns.player(evt.connection) {
        Ok(Some(player)) => player,
        _ => return,
    };

    let keystate = try_get!(player, mut keystate);

    last_key
        .insert(player, LastKeyTime(this_frame.0))
        .expect("Player doesn't exist");
    force
        .insert(player, ForcePlayerUpdate)
        .expect("Player doesn't exist");

    let state = match evt.packet.key {
        KeyCode::Up => &mut keystate.up,
        KeyCode::Down => &mut keystate.down,
        KeyCode::Left => &mut keystate.left,
        KeyCode::Right => &mut keystate.right,
        KeyCode::Fire => &mut keystate.fire,
        KeyCode::Special => &mut keystate.special,
    };

    *state = evt.packet.state;
}
