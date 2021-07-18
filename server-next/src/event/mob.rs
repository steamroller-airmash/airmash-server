use hecs::Entity;

/// Emitted when a new mob is spawned
#[derive(Clone, Copy, Debug)]
pub struct MobSpawn {
  pub mob: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MobDespawnType {
  Expired,
  PickUp,
}

/// Emitted when a mob is despawned.
#[derive(Copy, Clone, Debug)]
pub struct MobDespawn {
  pub mob: Entity,
  pub ty: MobDespawnType,
}

/// Emitted when a player picks up a mob.
#[derive(Clone, Copy, Debug)]
pub struct MobPickUp {
  pub mob: Entity,
  pub player: Entity,
}
