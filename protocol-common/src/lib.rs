#![feature(
	try_from,
	macro_at_most_once_rep,
	never_type,
	optin_builtin_traits
)]

#[macro_use]
extern crate lazy_static;
extern crate fnv;
#[macro_use]
extern crate dimensioned;

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", macro_use)]
extern crate serde;
#[cfg(feature = "specs")]
extern crate specs;
#[cfg(feature = "specs")]
#[cfg_attr(feature = "specs", macro_use)]
extern crate specs_derive;

#[macro_use]
mod detail;

mod consts;
mod enums;
mod packets;
mod traits;
mod types;

mod client_packet;
mod server_packet;

pub mod error;

pub use self::client_packet::*;
pub use self::enums::*;
pub use self::packets::*;
pub use self::server_packet::*;
pub use self::traits::*;
pub use self::types::*;
