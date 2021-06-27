use airmash_protocol::MobType;

use crate::component::*;
use crate::event::MissileTerrainCollision;
use crate::AirmashWorld;

#[handler]
fn send_despawn_packet(event: &MissileTerrainCollision, game: &mut AirmashWorld) {
  use crate::protocol::server::MobDespawnCoords;

  let query = game
    .world
    .query_one_mut::<(&MobType, &Position, &IsMissile)>(event.missile);
  let (&mob, pos, ..) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = MobDespawnCoords {
    id: event.missile.id() as _,
    ty: mob,
    pos: pos.0
  };
  game.send_to_visible(packet.pos, packet);
}
