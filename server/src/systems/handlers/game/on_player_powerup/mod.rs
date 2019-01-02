mod register;

mod send_player_powerup;
mod set_powerup_lifetime;
mod trigger_update;

pub use self::send_player_powerup::SendPlayerPowerup;
pub use self::set_powerup_lifetime::SetPowerupLifetime;
pub use self::trigger_update::TriggerUpdate;

pub use self::register::register;

use systems;

pub type AllPlayerPowerupSystems = (TriggerUpdate);

pub type KnownEventSources = (
	systems::admin::GivePowerup,
	systems::handlers::game::on_player_respawn::GiveShield,
	systems::powerups::Pickup,
);
