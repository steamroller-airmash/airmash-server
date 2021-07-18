use crate::component::*;
use crate::event::{MobDespawn, MobDespawnType};
use crate::AirmashGame;

#[handler]
fn send_packet(event: &MobDespawn, game: &mut AirmashGame) {
  use crate::protocol::{server::MobDespawn, DespawnType};

  let (&pos, _) = match game.world.query_one_mut::<(&Position, &IsMob)>(event.mob) {
    Ok(query) => query,
    Err(_) => return,
  };

  let ty = match event.ty {
    MobDespawnType::Expired => DespawnType::LifetimeEnded,
    MobDespawnType::PickUp => DespawnType::Collided,
  };

  game.send_to_visible(pos.0, MobDespawn {
    id: event.mob.id() as _,
    ty,
  });
}
