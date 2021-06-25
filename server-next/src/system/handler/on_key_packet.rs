use crate::event::KeyEvent;
use crate::event::PacketEvent;
use crate::protocol::client::Key;
use crate::AirmashWorld;

#[handler]
fn transform_key_event(event: &PacketEvent<Key>, game: &mut AirmashWorld) {
  game.dispatch(KeyEvent {
    player: event.entity,
    key: event.packet.key,
    state: event.packet.state,
  });
}
