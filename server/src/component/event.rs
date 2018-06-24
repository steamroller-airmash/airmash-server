use specs::*;

use std::any::Any;
use std::time::Instant;

use types::collision::Collision;
use utils::timer::TimerEventType;
use types::{ConnectionId, Position};

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
pub struct PlayerKilled {
	pub missile: Entity,
	pub player: Entity,
	pub killer: Entity,
	pub pos: Position
}
#[derive(Copy, Clone, Debug)]
pub struct PlayerRespawn(pub Entity);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PlayerTerrainCollision(pub Collision);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PlayerMissileCollision(pub Collision);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct MissileTerrainCollision(pub Collision);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PlayerPowerupCollision(pub Collision);

pub struct TimerEvent {
	pub ty: TimerEventType,
	pub instant: Instant,
	pub data: Option<Box<Any + Send + Sync>>
}

impl Default for TimerEvent {
	fn default() -> Self {
		use consts::timer::INVALID;

		Self {
			ty: *INVALID,
			instant: Instant::now(),
			data: None
		}
	}
}
