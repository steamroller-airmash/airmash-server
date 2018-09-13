use uuid::Uuid;

use specs::*;

use types::ConnectionId;

use std::time::Duration;

pub use protocol::FlagCode;
pub use protocol::MobType as Mob;
pub use protocol::PlaneType as Plane;
pub use protocol::PlayerStatus as Status;

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
