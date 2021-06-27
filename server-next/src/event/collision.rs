
use hecs::Entity;

#[derive(Copy, Clone, Debug)]
pub struct PlayerMissileCollision {
  pub player: Entity,
  pub missile: Entity
}
