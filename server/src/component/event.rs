
use specs::*;
use types::ConnectionId;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct ScoreDetailedEvent(pub ConnectionId);
#[derive(Copy, Clone, Debug, Default, Component)]
pub struct AckEvent(pub ConnectionId);

#[derive(Copy, Clone, Debug, Component)]
pub struct ScoreBoardTimerEvent(pub Instant);
#[derive(Copy, Clone, Debug, Component)]
pub struct AFKTimerEvent(pub Instant);
#[derive(Copy, Clone, Debug, Component)]
pub struct PingTimerEvent(pub Instant);
