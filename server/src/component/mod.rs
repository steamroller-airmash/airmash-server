//! All new components should be added here

mod packet_event;

pub mod channel;
pub mod collection;
pub mod collision;
pub mod counter;
pub mod event;
pub mod flag;
pub mod missile;
pub mod ratelimit;
pub mod reference;
pub mod stats;
pub mod time;

/// Utility module containing all components related to powerups
pub mod powerup {
	pub use super::{
		channel::{
			OnPlayerPowerup, OnPlayerPowerupCollision, OnPowerupDespawn,
			OnPowerupExpired as OnPowerupExpire, OnPowerupPickup, OnPowerupSpawn,
		},
		collision::PowerupGrid,
		event::{
			PlayerPowerup, PlayerPowerupCollision, PowerupDespawnEvent as PowerupDespawn,
			PowerupExpired as PowerupExpire, PowerupSpawnEvent as PowerupSpawn,
		},
		flag::IsPowerup,
	};
}

/// Utility module containing all components related to players
pub mod player {
	pub use super::{
		channel::{
			OnPlayerDespawn, OnPlayerHit, OnPlayerJoin, OnPlayerKilled, OnPlayerLeave,
			OnPlayerMissileCollision, OnPlayerMuted, OnPlayerPowerup, OnPlayerPowerupCollision,
			OnPlayerRepel, OnPlayerRespawn, OnPlayerSpectate, OnPlayerStealth,
			OnPlayerTerrainCollision, OnPlayerThrottled,
		},
		collision::{PlaneGrid, PlayerGrid},
		event::{
			PlayerDespawn, PlayerDespawnType, PlayerHit, PlayerJoin, PlayerKilled, PlayerLeave,
			PlayerMissileCollision, PlayerMute as PlayerMuted, PlayerPowerup,
			PlayerPowerupCollision, PlayerRepel, PlayerRespawn, PlayerRespawnPrevStatus,
			PlayerSpectate, PlayerStealth, PlayerTerrainCollision,
			PlayerThrottle as PlayerThrottled,
		},
		flag::{
			ForcePlayerUpdate, IsBoosting, IsChatMuted, IsChatThrottled, IsDead, IsPlayer,
			IsSpectating,
		},
	};
}
