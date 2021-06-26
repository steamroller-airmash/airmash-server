use crate::protocol::KeyCode;
use hecs::Entity;

mod missile;
mod packet;
mod player;

pub use self::missile::*;
pub use self::packet::*;
pub use self::player::*;

/// Emitted during server startup.
///
/// This is useful for registering resources at startup if so desired.
#[derive(Copy, Clone, Debug)]
pub struct ServerStartup;

#[derive(Copy, Clone, Debug)]
pub struct EntityDespawn {
  pub entity: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct EntitySpawn {
  pub entity: Entity,
}

/// An entity (player, missile, or mob) has left the horizon of another player.
#[derive(Copy, Clone, Debug)]
pub struct EventHorizon {
  pub player: Entity,
  /// The entity leaving/entering the horizon
  pub entity: Entity,
  /// The current state of the entity relative to the player
  pub in_horizon: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
  pub player: Entity,
  pub key: KeyCode,
  // True for pressed
  pub state: bool,
}

/// A player in a predator has started/stopped boosting
#[derive(Copy, Clone, Debug)]
pub struct EventBoost {
  pub player: Entity,
  pub boosting: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct EventRepel {
  pub player: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct EventStealth {
  pub player: Entity,
  pub stealthed: bool,
}
