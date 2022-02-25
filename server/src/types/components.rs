use uuid::Uuid;

use specs::*;

use crate::types::ConnectionId;

use std::time::Duration;

pub use crate::protocol::FlagCode;
pub use crate::protocol::MobType as Mob;
pub use crate::protocol::PlaneType as Plane;
pub use crate::protocol::PlayerStatus as Status;

#[derive(Clone, Debug, Default, Component, Eq, PartialEq, Hash)]
pub struct Name(pub String);
#[derive(Clone, Debug, Default, Component, Eq, PartialEq, Hash)]
pub struct Session(pub Option<Uuid>);
#[derive(Clone, Debug, Copy, Component, Default)]
pub struct AssociatedConnection(pub ConnectionId);

pub trait ToClock {
  fn to_clock(&self) -> u32;
}

impl ToClock for Duration {
  // Unit is hundredths of a millisecond. (1/1e5)
  fn to_clock(&self) -> u32 {
    ((self.as_secs() * 1_000_000) as u32 + self.subsec_micros()) / 10
  }
}
