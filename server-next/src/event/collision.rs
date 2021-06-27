
use hecs::Entity;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct PlayerMissileCollision {
  pub missile: Entity,
  pub players: SmallVec<[Entity; 1]>,
}

#[derive(Copy, Clone, Debug)]
pub struct MissileTerrainCollision {
  pub missile: Entity
}
