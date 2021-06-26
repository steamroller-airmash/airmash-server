//! AIRMASH Server

pub mod protocol {
  pub use airmash_protocol::*;
}

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
mod dispatch;
mod system;
mod world;
mod worldext;

pub mod component;
pub mod event;
pub mod network;
pub mod resource;
pub mod util;

pub use server_macros::handler;
pub use crate::protocol::Vector2;

pub use self::dispatch::{Event, EventDispatcher, EventHandler, HANDLERS};
pub use self::network::ConnectionMgr;
pub use self::world::AirmashWorld;
pub use self::worldext::FireMissileInfo;

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
  pub const HIGH: isize = 500;
  pub const MEDIUM: isize = 250;

  pub use crate::dispatch::DEFAULT_PRIORITY as DEFAULT;

  /// Priority of the handler that sends the login packet when a client attempts
  /// to log in.
  ///
  /// If you want to modify the state of the player before they get the login
  /// response then your event handler will need to have a greater priority than
  /// this.
  pub const LOGIN: isize = 1000;
}
