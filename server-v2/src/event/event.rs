use std::time::{Duration, Instant};

use crate::component::{Name, Session};
use crate::ecs::Entity;
use crate::protocol::*;
use crate::protocol::{FlagCode, PowerupType};
use crate::resource::socket::SocketId;

#[derive(Copy, Clone, Debug, Component)]
pub struct AFKTimerEvent(pub Instant);

#[derive(Clone, Debug)]
pub struct PlayerJoin {
    pub id: Entity,
    pub plane: PlaneType,
    pub team: Team,
    pub level: Level,
    pub name: Name,
    pub flag: FlagCode,
    pub session: Session,
    pub conn: SocketId,
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

// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
// pub struct PlayerTerrainCollision(pub Collision);
// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
// pub struct PlayerMissileCollision(pub Collision);
// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
// pub struct MissileTerrainCollision(pub Collision);
// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
// pub struct PlayerPowerupCollision(pub Collision);

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
    pub conn: SocketId,
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
    pub despawn: Option<Instant>,
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
    Powerup,
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
    pub just_spawned: bool,
}
