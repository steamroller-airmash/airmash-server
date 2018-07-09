//! Utility accessors for components
//! that are commonly used together.

mod clock;
mod isalive;
pub(crate) mod fire_missiles;

pub use self::clock::ReadClock;
pub use self::isalive::IsAlive;
pub use self::fire_missiles::FireMissiles;
