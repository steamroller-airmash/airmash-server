//! Event handlers for in-game events

mod onkilledcleanup;

mod register;

pub mod on_join;
pub mod on_missile_fire;
pub mod on_player_killed;
pub mod on_spectate_event;
pub mod timer;

pub use self::register::register;

pub use self::onkilledcleanup::PlayerKilledCleanup;
