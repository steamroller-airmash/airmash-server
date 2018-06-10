use uuid::Uuid;

use airmash_protocol::{self as protocol, FlagCode};
use specs::*;

use types::ConnectionId;

use std::time::Duration;

pub type Flag = FlagCode;
pub type Plane = protocol::PlaneType;
pub type Status = protocol::PlayerStatus;
pub type Mob = protocol::MobType;

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
	fn to_clock(&self) -> u32 {
		(self.as_secs() * 1_000_000) as u32 + self.subsec_micros() / 10
	}
}
