use crate::component::IsPlayer;
use crate::event::PlayerLeave;
use crate::AirmashWorld;

#[handler]
fn send_packet(event: &PlayerLeave, game: &mut AirmashWorld) {
  use crate::protocol::server as s;

  if let Err(_) = game.world.get::<IsPlayer>(event.player) {
    return;
  }

  game.send_to_all(s::PlayerLeave {
    id: event.player.id() as _,
  });
}
