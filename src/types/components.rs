
use uuid::Uuid;

use specs::*;
use airmash_protocol::{self as protocol, FlagCode};

use types::ConnectionId;

use std::time::Instant;

#[derive(Clone, Debug, Default, Component)]
pub struct Name(pub String);
#[derive(Clone, Debug, Default, Component)]
pub struct Session(pub Option<Uuid>);
#[derive(Clone, Copy, Debug, Component)]
pub struct Flag(pub FlagCode);
#[derive(Clone, Debug, Copy, Component)]
pub struct Plane(pub protocol::PlaneType);
#[derive(Clone, Debug, Copy, Component)]
pub struct Status(pub protocol::PlayerStatus);
#[derive(Clone, Debug, Copy, Component, Default)]
pub struct AssociatedConnection(pub ConnectionId);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastFrame(pub Instant);
#[derive(Clone, Debug, Copy, Component)]
pub struct ThisFrame(pub Instant);

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct ScoreDetailedEvent(pub ConnectionId);
#[derive(Copy, Clone, Debug, Default, Component)]
pub struct AckEvent(pub ConnectionId);

#[derive(Copy, Clone, Debug, Component)]
pub struct ScoreBoardTimerEvent(pub Instant);
#[derive(Copy, Clone, Debug, Component)]
pub struct AFKTimerEvent(pub Instant);

impl Default for LastFrame {
	fn default() -> Self {
		LastFrame(Instant::now())
	}
}
impl Default for ThisFrame {
	fn default() -> Self {
		ThisFrame(Instant::now())
	}
}

impl Default for Flag {
	fn default() -> Self {
		Flag(FlagCode::UnitedNations)
	}
}

impl Default for Plane {
	fn default() -> Self {
		Plane(protocol::PlaneType::Predator)
	}
}

impl Default for Status {
	fn default() -> Self {
		Status(protocol::PlayerStatus::Alive)
	}
}

