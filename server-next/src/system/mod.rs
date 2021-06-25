//! All systems and event handlers

use crate::AirmashWorld;

mod collision;
mod handler;
mod keys;
mod network;
mod physics;

#[allow(dead_code)]
pub fn update(game: &mut AirmashWorld) {
  self::physics::frame_update(game);
  self::collision::generate_lookups(game);

  // Note: most events will happen here
  self::network::process_packets(game);
}
