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

mod dispatch;
mod system;
mod world;
mod consts;

pub mod component;
pub mod event;
pub mod resource;
pub mod util;
pub mod network;

pub use self::dispatch::{Event, EventDispatcher, EventHandler, HANDLERS};
pub use self::world::AirmashWorld;
