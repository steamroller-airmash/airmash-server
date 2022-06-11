use hecs::Entity;
use smallvec::SmallVec;

/// A collision occurred between a missile and any number of players.
#[derive(Clone, Debug)]
pub struct PlayerMissileCollision {
  pub missile: Entity,
  pub players: SmallVec<[Entity; 1]>,
}

/// A collision occurred between a mob and a player.
#[derive(Copy, Clone, Debug)]
pub struct PlayerMobCollision {
  pub mob: Entity,
  pub player: Entity,
}

/// A collision occurred between a missile and the terrain.
#[derive(Copy, Clone, Debug)]
pub struct MissileTerrainCollision {
  pub missile: Entity,
}
