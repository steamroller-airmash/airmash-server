use std::time::Duration;

use airmash_protocol::{PlaneType, PowerupType};
use hecs::Entity;
use smallvec::SmallVec;

#[derive(Clone, Copy, Debug)]
pub struct PlayerJoin {
  pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerLeave {
  pub player: Entity,
}

#[derive(Clone, Debug)]
pub struct PlayerFire {
  pub player: Entity,
  pub missiles: SmallVec<[Entity; 3]>,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerKilled {
  pub player: Entity,
  pub missile: Entity,
  pub killer: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerRespawn {
  pub player: Entity,
  /// Whether the player was alive when they respawned
  pub alive: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerChangePlane {
  pub player: Entity,
  pub old_plane: PlaneType,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerPowerup {
  pub player: Entity,
  pub ty: PowerupType,
  pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct PlayerRepel {
  pub player: Entity,
  pub repelled_players: SmallVec<[Entity; 4]>,
  pub repelled_missiles: SmallVec<[Entity; 4]>,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerSpectate {
  pub player: Entity,
  pub was_alive: bool,
}
