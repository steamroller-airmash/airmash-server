//!

#[macro_use]
extern crate serde;

mod util;
mod missile;
mod plane;
mod special;

pub use self::missile::MissilePrototype;
pub use self::plane::PlanePrototype;
pub use self::special::*;
