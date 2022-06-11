use crate::component::*;
use crate::event::KeyEvent;
use crate::event::PacketEvent;
use crate::protocol::client::Key;
use crate::AirmashGame;

#[handler]
fn transform_key_event(event: &PacketEvent<Key>, game: &mut AirmashGame) {
  let (alive, _) = match game
    .world
    .query_one_mut::<(&IsAlive, &IsPlayer)>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if !alive.0 {
    return;
  }

  game.dispatch(KeyEvent {
    player: event.entity,
    key: event.packet.key,
    state: event.packet.state,
  });
}
