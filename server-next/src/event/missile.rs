use hecs::Entity;

/// The reason that the missile despawned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MissileDespawnType {
  HitPlayer,
  HitTerrain,
  LifetimeEnded,
}

/// A missile despawned.
#[derive(Copy, Clone, Debug)]
pub struct MissileDespawn {
  pub missile: Entity,
  pub ty: MissileDespawnType,
}
