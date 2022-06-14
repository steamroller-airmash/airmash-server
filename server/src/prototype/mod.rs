//! Prototypes for objects within airmash.
//!
//! These define the properties that planes, missiles, mobs, specials, and more
//! will have within airmash. This is to allow for custom configurations to be
//! created without having to modify code.

mod missile;
mod plane;
mod special;

pub use self::missile::MissilePrototype;
pub use self::plane::PlanePrototype;
pub use self::special::*;
