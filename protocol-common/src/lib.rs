#![feature(try_from, optin_builtin_traits, never_type)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate fnv;
extern crate specs;
#[macro_use]
extern crate dimensioned;

#[macro_use]
mod detail;

mod consts;
mod enums;
mod packets;
mod types;

mod client_packet;
mod server_packet;

pub mod error;

pub use self::enums::*;
pub use self::packets::*;
pub use self::types::*;
