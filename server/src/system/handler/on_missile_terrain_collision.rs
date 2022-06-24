use crate::component::*;
use crate::config::MissilePrototypeRef;
use crate::event::MissileTerrainCollision;
use crate::AirmashGame;

#[handler]
fn send_despawn_packet(event: &MissileTerrainCollision, game: &mut AirmashGame) {
  use crate::protocol::server::MobDespawnCoords;

  let query = game
    .world
    .query_one_mut::<(&MissilePrototypeRef, &Position, &IsMissile)>(event.missile);
  let (&mob, pos, ..) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = MobDespawnCoords {
    id: event.missile.id() as _,
    ty: mob.server_type,
    pos: pos.into(),
  };
  game.send_to_visible(packet.pos.into(), packet);
}
