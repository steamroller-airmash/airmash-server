use crate::component::*;
use crate::config::PlanePrototypeRef;
use crate::event::PlayerChangePlane;
use crate::AirmashGame;

#[handler]
fn send_packet(event: &PlayerChangePlane, game: &mut AirmashGame) {
  use crate::protocol::server as s;

  let (&plane, _) = match game
    .world
    .query_one_mut::<(&PlanePrototypeRef, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  game.send_to_all(s::PlayerType {
    id: event.player.id() as _,
    ty: plane.server_type,
  });
}
