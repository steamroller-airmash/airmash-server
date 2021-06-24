//! All systems and event handlers

use crate::AirmashWorld;

mod keys;
mod physics;
mod network;
mod collision;

pub fn update(game: &mut AirmashWorld) {
  self::physics::frame_update(game);
  self::network::process_packets(game);
}
