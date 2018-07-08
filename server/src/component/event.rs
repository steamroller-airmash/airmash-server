use specs::*;

use std::any::Any;
use std::time::Instant;

use protocol::FlagCode;
use types::collision::Collision;
use types::*;
use utils::timer::TimerEventType;

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct ScoreDetailedEvent(pub ConnectionId);
#[derive(Copy, Clone, Debug, Default, Component)]
pub struct AckEvent(pub ConnectionId);

#[derive(Copy, Clone, Debug, Component)]
pub struct AFKTimerEvent(pub Instant);

#[derive(Clone, Debug)]
pub struct PlayerJoin {
	pub id: Entity,
	pub plane: Plane,
	pub team: Team,
	pub level: Level,
	pub name: Name,
	pub flag: FlagCode,
	pub session: Session,
	pub conn: ConnectionId,
}
#[derive(Copy, Clone, Debug)]
pub struct PlayerLeave(pub Entity);
#[derive(Copy, Clone, Debug)]
pub struct PlayerKilled {
	pub missile: Entity,
	pub player: Entity,
	pub killer: Entity,
	pub pos: Position,
}
#[derive(Copy, Clone, Debug)]
pub struct PlayerRespawn(pub Entity);

#[derive(Copy, Clone, Debug)]
pub struct PlayerSpectate {
	pub player: Entity,
	pub target: Option<Entity>,
	pub is_dead: bool,
	pub is_spec: bool,
}

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
	pub data: Option<Box<Any + Send + Sync>>,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerStealth {
	pub stealthed: bool,
	pub player: Entity,
}
#[derive(Copy, Clone, Debug)]
pub struct PlayerRepel {
	pub player: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct MissileFire {
	pub player: Entity,
	pub missile: Entity,
}

impl Default for TimerEvent {
	fn default() -> Self {
		use consts::timer::INVALID;

		Self {
			ty: *INVALID,
			instant: Instant::now(),
			data: None,
		}
	}
}
