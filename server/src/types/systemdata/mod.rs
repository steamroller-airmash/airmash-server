//! Utility accessors for components
//! that are commonly used together.

mod clock;
mod connections;
pub(crate) mod fire_missiles;
mod isalive;

pub use self::clock::ReadClock;
pub use self::connections::{Connections, ConnectionsMut, ConnectionsNoTeams};
pub use self::fire_missiles::FireMissiles;
pub use self::isalive::IsAlive;
