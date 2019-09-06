//! Event handlers for in-game events

mod register;

pub mod on_chat_throttled;
pub mod on_enter_horizon;
pub mod on_join;
pub mod on_leave;
pub mod on_leave_horizon;
pub mod on_missile_despawn;
pub mod on_missile_fire;
pub mod on_player_despawn;
pub mod on_player_hit;
pub mod on_player_killed;
pub mod on_player_powerup;
pub mod on_player_respawn;
pub mod on_spectate_event;
pub mod timer;

pub use self::register::register;

// Modules that just forward to ones elsewhere

pub mod on_powerup_despawn {
	pub use crate::systems::powerups::on_despawn::*;

	use crate::component::powerup::PowerupDespawn;
	use crate::utils::EventSources;
	pub type KnownEventSources = EventSources<PowerupDespawn>;
}

pub mod on_powerup_spawn {
	pub use crate::systems::powerups::on_spawn::*;

	use crate::component::powerup::PowerupSpawn;
	use crate::utils::EventSources;
	pub type KnownEventSources = EventSources<PowerupSpawn>;
}

pub mod on_powerup_expire {
	pub use crate::systems::powerups::on_expire::*;

	use crate::component::powerup::PowerupExpire;
	use crate::utils::EventSources;
	pub type KnownEventSources = EventSources<PowerupExpire>;
}
