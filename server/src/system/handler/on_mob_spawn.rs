use airmash_protocol::server::MobUpdateStationary;

use crate::component::*;
use crate::config::MobPrototypeRef;
use crate::event::MobSpawn;
use crate::AirmashGame;

#[handler]
fn send_packet(event: &MobSpawn, game: &mut AirmashGame) {
  let (&mob, &pos, _) = match game
    .world
    .query_one_mut::<(&MobPrototypeRef, &Position, &IsMob)>(event.mob)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  game.send_to_visible(
    pos.0,
    MobUpdateStationary {
      id: event.mob.id() as _,
      ty: mob.server_type,
      pos: pos.0,
    },
  );
}
