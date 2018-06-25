//! Event handlers for in-game events

mod onkilledmessage;
mod onkilledcleanup;
mod onrespawntimer;

mod register;

pub use self::register::register;

pub use self::onkilledmessage::PlayerKilledMessage;
pub use self::onkilledcleanup::PlayerKilledCleanup;
