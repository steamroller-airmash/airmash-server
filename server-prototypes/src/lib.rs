//!

#[macro_use]
extern crate serde;

mod error;
mod game;
mod missile;
mod plane;
mod special;
mod util;

#[cfg(feature = "script")]
mod script;

pub use self::game::GamePrototype;
pub use self::missile::MissilePrototype;
pub use self::plane::PlanePrototype;
pub use self::special::*;
pub use self::error::{ValidationError, Path, Segment};
