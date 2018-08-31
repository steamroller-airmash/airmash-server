#![feature(try_from, optin_builtin_traits, never_type)]

#[macro_use]
extern crate lazy_static;
extern crate fnv;
#[macro_use]
extern crate dimensioned;

#[cfg(features = "serde")]
#[cfg_attr(features = "serde", macro_use)]
extern crate serde;
#[cfg(features = "specs")]
extern crate specs;

#[macro_use]
mod detail;

mod consts;
mod enums;
mod packets;
mod types;

mod client_packet;
mod server_packet;

pub mod error;

pub use self::client_packet::*;
pub use self::enums::*;
pub use self::packets::*;
pub use self::server_packet::*;
pub use self::types::*;
