use hecs::Entity;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MissileDespawnType {
  HitPlayer,
  HitTerrain,
  LifetimeEnded,
}

#[derive(Copy, Clone, Debug)]
pub struct MissileDespawn {
  pub missile: Entity,
  pub ty: MissileDespawnType,
}
