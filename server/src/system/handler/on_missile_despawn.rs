use airmash_protocol::MobType;

use crate::component::*;
use crate::event::{MissileDespawn, MissileDespawnType};
use crate::AirmashGame;

#[handler]
fn send_despawn_packet(event: &MissileDespawn, game: &mut AirmashGame) {
  use crate::protocol::{server as s, DespawnType};

  let (&pos, &mob, ..) = match game
    .world
    .query_one_mut::<(&Position, &MobType, &IsMissile)>(event.missile)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let ty = match event.ty {
    MissileDespawnType::HitPlayer => DespawnType::Collided,
    MissileDespawnType::HitTerrain => DespawnType::Collided,
    MissileDespawnType::LifetimeEnded => DespawnType::LifetimeEnded,
  };

  if event.ty != MissileDespawnType::LifetimeEnded {
    game.send_to_visible(
      pos.0,
      s::MobDespawnCoords {
        id: event.missile.id() as _,
        pos: pos.0,
        ty: mob,
      },
    );
  }

  game.send_to_visible(
    pos.0,
    s::MobDespawn {
      id: event.missile.id() as _,
      ty,
    },
  );
}
