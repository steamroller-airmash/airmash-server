
use hecs::Entity;
use crate::protocol::KeyCode;

mod packet;
mod player;

pub use self::packet::*;
pub use self::player::*;

#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
  pub player: Entity,
  pub key: KeyCode,
  // True for pressed
  pub state: bool
}

/// A player in a predator has started/stopped boosting
#[derive(Copy, Clone, Debug)]
pub struct EventBoost {
  pub player: Entity,
  pub boosting: bool
}

#[derive(Copy, Clone, Debug)]
pub struct EventRepel {
  pub player: Entity
}

#[derive(Copy, Clone, Debug)]
pub struct EventStealth {
  pub player: Entity,
  pub stealthed: bool
}
