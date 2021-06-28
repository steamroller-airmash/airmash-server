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
  pub player: Entity
}
