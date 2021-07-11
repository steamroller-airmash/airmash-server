use airmash::Entity;

use std::time::Instant;

#[derive(Copy, Clone, Debug, Default)]
pub struct IsFlag;

#[derive(Copy, Clone, Debug, Default)]
pub struct FlagCarrier(pub Option<Entity>);

#[derive(Copy, Clone, Debug)]
pub struct LastDrop {
  pub player: Option<Entity>,
  pub time: Instant,
}

#[derive(Copy, Clone, Debug)]
pub struct Flags {
  pub red: Entity,
  pub blue: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct LastReturnTime(pub Instant);
