use specs::*;
use std::time::Instant;
use types::ConnectionId;

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

#[derive(Copy, Clone, Debug)]
pub struct PlayerJoin(pub Entity);
#[derive(Copy, Clone, Debug)]
pub struct PlayerLeave(pub Entity);
#[derive(Copy, Clone, Debug)]
pub struct PlayerKilled(pub Entity);

