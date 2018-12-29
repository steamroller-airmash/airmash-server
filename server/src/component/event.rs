use specs::*;

use std::any::Any;
use std::time::{Duration, Instant};

use protocol::client::*;
use protocol::{FlagCode, PowerupType};
use types::collision::Collision;
pub use types::event::{ConnectionClose, ConnectionOpen, Message};
use types::*;
pub use utils::timer::TimerEventType;

pub use super::packet_event::PacketEvent;

pub type BinaryEvent = (ConnectionId, Vec<u8>);
pub type LoginEvent = (ConnectionId, Login);
pub type BackupEvent = (ConnectionId, Backup);
pub type CommandEvent = (ConnectionId, Command);
pub type HorizonEvent = (ConnectionId, Horizon);
pub type KeyEvent = (ConnectionId, Key);
pub type PongEvent = PacketEvent<Pong>;
pub type ChatEvent = (ConnectionId, Chat);
pub type SayEvent = (ConnectionId, Say);
pub type TeamChatEvent = (ConnectionId, TeamChat);
pub type WhisperEvent = (ConnectionId, Whisper);
pub type VotemuteEvent = (ConnectionId, VoteMute);
pub type LocalPingEvent = (ConnectionId, LocalPing);

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

/// The status of the player when they respawned.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PlayerRespawnPrevStatus {
	Dead,
	Alive,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerRespawn {
	pub player: Entity,
	pub prev_status: PlayerRespawnPrevStatus,
}
#[derive(Copy, Clone, Debug)]
pub struct PlayerHit {
	pub player: Entity,
	pub missile: Entity,
}

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
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PlayerUpgradeCollision(pub Collision);

pub struct TimerEvent {
	pub ty: TimerEventType,
	pub instant: Instant,
	pub data: Option<Box<Any + Send + Sync + 'static>>,
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

#[derive(Clone, Debug)]
pub struct MissileFire {
	pub player: Entity,
	pub missiles: Vec<Entity>,
}

#[derive(Clone, Debug)]
pub struct PlayerMute {
	pub player: Entity,
}

#[derive(Clone, Debug)]
pub struct PlayerThrottle {
	pub player: Entity,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ChatEventType {
	Public,
	Whisper(u16),
	Team,
	Say,
}

#[derive(Clone, Debug)]
pub struct AnyChatEvent {
	pub ty: ChatEventType,
	pub text: String,
	pub conn: ConnectionId,
}

#[derive(Copy, Clone, Debug)]
pub struct PowerupPickupEvent {
	pub pos: Position,
	pub upgrade: Entity,
	pub player: Entity,
}

/// Note: Includes upgrades
#[derive(Copy, Clone, Debug)]
pub struct PowerupSpawnEvent {
	pub mob: Entity,
	pub ty: Mob,
	pub pos: Position,
	pub despawn: Instant,
}

/// Note: Includes upgrades
#[derive(Copy, Clone, Debug)]
pub struct PowerupDespawnEvent {
	pub mob: Entity,
	pub ty: Mob,
	pub pos: Position,
	/// The player that picked up this powerup
	/// (if it was picked up by a player)
	pub player: Option<Entity>,
}

#[derive(Copy, Clone, Debug)]
pub struct PowerupExpired {
	pub player: Entity,
	pub ty: PowerupType,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerPowerup {
	pub player: Entity,
	pub duration: Duration,
	pub ty: PowerupType,
}

/// All the different reasons a player could
/// have for despawning.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlayerDespawnType {
	Disconnect,
	Killed,
	Spectate,
	Respawn,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerDespawn {
	pub player: Entity,
	pub ty: PlayerDespawnType,
	pub pos: Position,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MissileDespawnType {
	HitPlayer,
	HitTerrain,
	LifetimeEnded,
}

#[derive(Copy, Clone, Debug)]
pub struct MissileDespawn {
	pub missile: Entity,
	pub ty: MissileDespawnType,
	pub pos: Position,
	pub mob: Mob,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum EntityType {
	Player,
	Missile,
	Upgrade,
}

#[derive(Copy, Clone, Debug)]
pub struct LeaveHorizon {
	/// The player who previously had the other entity
	/// within their horizon.
	pub player: Entity,
	/// The entity that is now outside the horizon
	/// of `player`.
	pub left: Entity,
	pub left_ty: EntityType,
}

#[derive(Copy, Clone, Debug)]
pub struct EnterHorizon {
	pub player: Entity,
	pub entered: Entity,
	pub entered_ty: EntityType,
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
