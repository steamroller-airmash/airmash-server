//! Event handlers for in-game events

mod onkilledcleanup;
mod onkilledmessage;
mod onrespawntimer;

mod register;

pub use self::register::register;

pub use self::onkilledcleanup::PlayerKilledCleanup;
pub use self::onkilledmessage::PlayerKilledMessage;
pub use self::onrespawntimer::OnRespawnTimer;
