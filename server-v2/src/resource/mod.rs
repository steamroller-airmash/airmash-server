//! Resources used within the airmash server.
//!

mod config;
mod connections;

pub mod builtin;
pub mod collision;
pub mod packet;
pub mod socket;

pub use self::builtin::{CurrentFrame, LastFrame, PlayerCount, StartTime};
pub use self::config::Config;
pub use self::connections::{Connections, NonexistantSocketError};
pub use self::inner::PlayerNames;

mod inner {
    use crate::ecs::Entity;
    use fxhash::FxHashMap;

    #[derive(Clone, Debug, Default)]
    pub struct PlayerNames(pub FxHashMap<String, Entity>);
}

pub mod channel {
    use crate::event::collision::*;
    use crate::event::*;
    use shrev::EventChannel;

    pub type OnAFKTimerEvent = EventChannel<AFKTimerEvent>;
    pub type OnPlayerJoin = EventChannel<PlayerJoin>;
    pub type OnPlayerLeave = EventChannel<PlayerLeave>;
    pub type OnPlayerKilled = EventChannel<PlayerKilled>;
    pub type OnPlayerPowerup = EventChannel<PlayerPowerup>;

    pub type OnPlayerTerrainCollision = EventChannel<PlayerTerrainCollision>;
}
