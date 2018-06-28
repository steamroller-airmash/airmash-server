//! Event handlers for in-game events

mod onkilledcleanup;
mod onkilledmessage;

mod register;

pub mod on_spectate_event;
pub mod on_player_killed;
pub mod timer;

pub use self::register::register;

pub use self::onkilledcleanup::PlayerKilledCleanup;
pub use self::onkilledmessage::PlayerKilledMessage;
