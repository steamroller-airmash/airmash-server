//! All systems and event handlers

use crate::AirmashWorld;

mod collision;
mod handler;
mod keys;
mod network;
mod physics;
mod regen;
mod specials;

#[allow(dead_code)]
pub fn update(game: &mut AirmashWorld) {
  self::physics::update(game);
  self::regen::update(game);
  self::specials::update(game);

  self::collision::update(game);

  // Note: most events will happen here
  self::network::process_packets(game);
}
