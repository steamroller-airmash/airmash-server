//!

#[macro_use]
extern crate serde;

mod common;
mod config;
mod error;
mod game;
mod missile;
mod plane;
mod special;
mod util;

#[cfg(feature = "script")]
mod script;

pub(crate) use self::error::ValidationExt;

pub use self::common::GameConfigCommon;
pub use self::config::GameConfig;
pub use self::error::{Path, Segment, ValidationError};
pub use self::game::GamePrototype;
pub use self::missile::MissilePrototype;
pub use self::plane::PlanePrototype;
pub use self::special::*;
