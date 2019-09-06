mod register;

mod send_player_powerup;
mod trigger_update;

pub use self::send_player_powerup::SendPlayerPowerup;
pub use self::trigger_update::TriggerUpdate;

pub use self::register::register;

pub type AllPlayerPowerupSystems = (SendPlayerPowerup, TriggerUpdate);
pub type KnownEventSources = crate::utils::EventSources<crate::component::event::PlayerPowerup>;
