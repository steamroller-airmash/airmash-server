use uuid::Uuid;

use airmash_protocol::{self as protocol, FlagCode};
use specs::*;

use types::ConnectionId;

use std::time::{Duration, Instant};

pub type Flag = FlagCode;
pub type Plane = protocol::PlaneType;
pub type Status = protocol::PlayerStatus;

#[derive(Clone, Debug, Default, Component, Eq, PartialEq, Hash)]
pub struct Name(pub String);
#[derive(Clone, Debug, Default, Component, Eq, PartialEq, Hash)]
pub struct Session(pub Option<Uuid>);
#[derive(Clone, Debug, Copy, Component, Default)]
pub struct AssociatedConnection(pub ConnectionId);

#[derive(Clone, Debug, Copy, Component)]
pub struct LastFrame(pub Instant);
#[derive(Clone, Debug, Copy, Component)]
pub struct ThisFrame(pub Instant);
#[derive(Clone, Debug, Copy, Component)]
pub struct StartTime(pub Instant);
#[derive(Clone, Debug, Copy, Component)]
pub struct LastUpdate(pub Instant);

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
impl Default for StartTime {
    fn default() -> Self {
        StartTime(Instant::now())
    }
}
impl Default for LastUpdate {
    fn default() -> Self {
        LastUpdate(Instant::now())
    }
}

pub trait ToClock {
    fn to_clock(&self) -> u32;
}

impl ToClock for Duration {
    fn to_clock(&self) -> u32 {
        (self.as_secs() * 1_000_000) as u32 + self.subsec_micros() / 10
    }
}
