//! AIRMASH Server

/// A re-export of the entirety of the `airmash_protocol` crate.
pub mod protocol {
  pub use airmash_protocol::*;
}

pub mod config {
  pub use server_config::*;

  pub type GameConfigRef = &'static GameConfig;
  pub type PlanePrototypeRef = &'static PlanePrototype<'static, PtrRef>;
  pub type MissilePrototypeRef = &'static MissilePrototype;
  pub type SpecialPrototypeRef = &'static SpecialPrototype<'static, PtrRef>;
  pub type PowerupPrototypeRef = &'static PowerupPrototype;
  pub type MobPrototypeRef = &'static MobPrototype<'static, PtrRef>;
}

pub extern crate hecs;
pub extern crate nalgebra;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate server_macros;

extern crate self as airmash_server;

#[macro_use]
mod macros;

mod consts;
mod defaults;
mod dispatch;
mod mock;
mod task;
mod world;
mod worldext;

pub mod component;
pub mod event;
pub mod network;
pub mod resource;
pub mod system;
pub mod util;

pub use hecs::Entity;
pub use nalgebra::Vector2;
pub use server_macros::handler;

pub use self::dispatch::{Event, EventDispatcher, EventHandler};
pub use self::task::{GameRef, TaskScheduler};
pub use self::world::{AirmashGame, Resources};
pub use self::worldext::{EntitySetBuilder, FireMissileInfo};

/// Exports needed by the handler macro.
#[doc(hidden)]
pub mod _exports {
  pub use crate::dispatch::{EventDispatcher, AIRMASH_EVENT_HANDLERS};
  pub extern crate linkme;
}

/// Reexports of common items that are often needed when writing server code.
pub mod prelude {
  pub use crate::{handler, AirmashGame, Entity};
}

/// Notable priorities for event handlers.
///
/// Most systems will have the default priority (0) but in some cases the order
/// that things happen is really important. If those cases are relevant
/// externally then their priorities will be included here.
///
/// > ### Note: How priorities work
/// > When the event dispatcher recieves an event it will execute the handlers
/// > in order of decreasing priority. Handlers with the same priority will be
/// > executed in an unspecified order so if you need a handler to execute
/// > before another then it must have a higher priority.
pub mod priority {
  pub const HIGH: i32 = 500;
  pub const MEDIUM: i32 = 250;

  pub const CLEANUP: i32 = -500;

  pub use crate::dispatch::DEFAULT_PRIORITY as DEFAULT;

  /// Priority of the handler that sends the login packet when a client attempts
  /// to log in.
  ///
  /// If you want to modify the state of the player before they get the login
  /// response then your event handler will need to have a greater priority than
  /// this.
  pub const LOGIN: i32 = 1000;
  pub const PRE_LOGIN: i32 = 1500;
}

/// Utilities to help with writing tests for server functionality.
pub mod test {
  pub use crate::mock::*;
}
