use crate::component::*;
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

#[handler]
fn remove_name(event: &PlayerLeave, game: &mut AirmashWorld) {
  use crate::resource::TakenNames;

  let mut taken_names = game.resources.write::<TakenNames>();
  let name = match game.world.get::<Name>(event.player) {
    Ok(name) => name,
    Err(_) => return,
  };

  taken_names.remove(&name.0);
}
