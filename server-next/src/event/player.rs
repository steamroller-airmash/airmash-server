use std::time::Duration;

use airmash_protocol::{PlaneType, PowerupType};
use hecs::Entity;
use smallvec::SmallVec;

/// A new player has joined the game.
#[derive(Clone, Copy, Debug)]
pub struct PlayerJoin {
  pub player: Entity,
}

/// A player has left the game.
///
/// Note that this event is emitted before the player's entity is despawned.
#[derive(Clone, Copy, Debug)]
pub struct PlayerLeave {
  pub player: Entity,
}

/// A player has fired missiles.
#[derive(Clone, Debug)]
pub struct PlayerFire {
  pub player: Entity,
  pub missiles: SmallVec<[Entity; 3]>,
}

/// A player has been killed by another player.
///
/// Note that the player who fired the missile may no longer be on the server so
/// `killer` is an option.
#[derive(Copy, Clone, Debug)]
pub struct PlayerKilled {
  pub player: Entity,
  pub missile: Entity,
  pub killer: Option<Entity>,
}

/// A player has respawned.
#[derive(Copy, Clone, Debug)]
pub struct PlayerRespawn {
  pub player: Entity,
  /// Whether the player was alive when they respawned
  pub alive: bool,
}

/// A player has switched their current plane.
#[derive(Copy, Clone, Debug)]
pub struct PlayerChangePlane {
  pub player: Entity,
  pub old_plane: PlaneType,
}

/// A player has obtained a powerup.
#[derive(Copy, Clone, Debug)]
pub struct PlayerPowerup {
  pub player: Entity,
  pub ty: PowerupType,
  pub duration: Duration,
}

/// A goliath has used their special.
#[derive(Clone, Debug)]
pub struct PlayerRepel {
  pub player: Entity,
  pub repelled_players: SmallVec<[Entity; 4]>,
  pub repelled_missiles: SmallVec<[Entity; 4]>,
}

/// A player has entered spectate mode.
#[derive(Copy, Clone, Debug)]
pub struct PlayerSpectate {
  pub player: Entity,
  pub was_alive: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerHit {
  pub player: Entity,
  pub missile: Entity,
  pub damage: f32,
  pub attacker: Option<Entity>,
}
