use airmash_protocol::PlaneType;

use crate::component::*;
use crate::event::EventBounce;
use crate::world::AirmashWorld;

#[handler]
fn send_bounce_packet(event: &EventBounce, game: &mut AirmashWorld) {
  let clock = crate::util::get_current_clock(game);

  let query = game.world.query_one_mut::<(
    &Position,
    &Velocity,
    &Rotation,
    &KeyState,
    &PlaneType,
    &Team,
    &SpecialActive,
  )>(event.player);
  let (pos, vel, rot, keystate, plane, team, active) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = crate::protocol::server::EventBounce {
    clock,
    id: event.player.id() as _,
    pos: pos.0,
    rot: rot.0,
    speed: vel.0,
    keystate: keystate.to_server(plane, active),
  };

  let team = team.0;
  drop(query);

  if keystate.stealthed {
    game.send_to_team_visible(team, packet.pos, packet);
  } else {
    game.send_to_visible(packet.pos, packet);
  }
}
