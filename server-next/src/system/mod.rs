//! All systems and event handlers

use crate::AirmashWorld;

mod collision;
mod despawn;
mod handler;
mod keys;
mod network;
mod physics;
mod regen;
mod specials;
mod visibility;

#[allow(dead_code)]
pub fn update(game: &mut AirmashWorld) {
  self::physics::update(game);
  self::regen::update(game);
  self::specials::update(game);

  self::collision::generate_collision_lookups(game);
  self::visibility::generate_horizon_events(game);
  self::collision::check_collisions(game);

  // Note: most events will happen here
  self::network::process_packets(game);

  self::keys::update(game);
  self::despawn::update(game);

  cull_zombies(game);
}

/// Reusing an id soon after it was created causes problems with the airmash web
/// client. To avoid this we insert a placeholder entity when despawning other
/// entities. This function is responsible for deleting them after a certain
/// amount of time has passed.
///
/// Since nothing should be interacting with these entities it emits no events.
fn cull_zombies(game: &mut AirmashWorld) {
  use crate::component::{Expiry, IsZombie};
  use smallvec::SmallVec;

  let this_frame = game.this_frame();

  let dead = game
    .world
    .query_mut::<&Expiry>()
    .with::<IsZombie>()
    .into_iter()
    .filter(|(_, expiry)| expiry.0 < this_frame)
    .map(|(ent, _)| ent)
    .collect::<SmallVec<[_; 32]>>();

  for entity in dead {
    let _ = game.world.despawn(entity);
  }
}