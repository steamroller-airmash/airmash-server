#![feature(try_from, never_type)]

extern crate protocol_common;

#[macro_use]
mod macros;

mod client;
mod server;

mod error;
mod funcs;
mod protocol;
mod serde;

pub use error::*;
pub use protocol::ProtocolV5;
