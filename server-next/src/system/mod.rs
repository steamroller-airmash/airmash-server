//! All systems and event handlers.
//!
//! Most of this module is bundled up within the [`update`] function. However,
//! it also exposes some optional systems that are not registered by default but
//! may be useful for certain game modes.

use crate::AirmashGame;

pub mod ffa;
pub mod ctf;

mod admin;
mod collision;
mod despawn;
mod handler;
mod keys;
mod network;
mod physics;
mod ping;
mod powerups;
mod regen;
mod scoreboard;
mod specials;
mod visibility;

/// Main airmash update loop.
///
/// This is the main method that contains all the work done within a single
/// frame of the airmash engine. Generally it is not something you should have
/// to call as it will be called as a part of [`AirmashGame::run_once`] or
/// [`AirmashGame::run_until_shutdown`].
///
/// [`AirmashGame::run_once`]: crate::AirmashGame::run_once
/// [`AirmashGame::run_until_shutdown`]: crate::AirmashGame::run_until_shutdown
pub fn update(game: &mut AirmashGame) {
  use crate::event::{Frame, FrameEnd, FrameStart};

  game.dispatch(FrameStart);

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
  self::powerups::update(game);
  self::scoreboard::update(game);
  self::ping::update(game);

  game.dispatch(Frame);

  update_tasks(game);
  cull_zombies(game);

  game.dispatch(FrameEnd);
}

/// Reusing an id soon after it was created causes problems with the airmash web
/// client. To avoid this we insert a placeholder entity when despawning other
/// entities. This function is responsible for deleting them after a certain
/// amount of time has passed.
///
/// Since nothing should be interacting with these entities it emits no events.
fn cull_zombies(game: &mut AirmashGame) {
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

/// This system is responsible for running delayed tasks that have been
/// scheduled by other code.
fn update_tasks(game: &mut AirmashGame) {
  use crate::resource::TaskScheduler;

  let task_sched = game.resources.read::<TaskScheduler>().clone();
  task_sched.update(game);
}
