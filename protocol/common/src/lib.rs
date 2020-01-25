#![feature(optin_builtin_traits, const_generics)]
#![cfg_attr(test, feature(test))]
#![allow(incomplete_features)]

//! Incomplete protocol interface for [AIRMASH][0]. This crate provides
//! a strongly typed interface for communicating with an airmash server.
//! Since there can (in the future) be multiple protocol versions, this
//! crate must be used with another crate such as [airmash-protocol-v5][1]
//! that provides a serialization layer.
//!
//! # Library Usage
//! Once you have gotten an instance of [`Protocol`][2], it can be used to
//! serialize and deserialize to and from the wire representation of that
//! protocol version.
//!
//! ```
//! # extern crate airmash_protocol;
//! # use airmash_protocol::*;
//! # use std::mem;
//! # use std::error::Error;
//! # // Just some error type that implements Error
//! # use std::fmt::Error as FmtError;
//! #
//! # fn main() -> Result<(), Box<Error>> {
//! # // This is required since if this example were to actually run
//! # // it would immediately cause undefined behaviour.
//! # return Ok(());
//! # // This crate doesn't have any protocol implementations, but we can pretend it does by
//! # // not actually being able to make them
//! # let protocol_from_elsewhere: Box<Protocol<SerializeError = FmtError, DeserializeError = FmtError>> = unsafe{ mem::uninitialized() };
//! # let bytes_from_elsewhere: Vec<u8> = unsafe{ mem::uninitialized() };
//! let protocol = protocol_from_elsewhere;
//! let bytes = bytes_from_elsewhere;
//!
//! // To deserialize a packet from the server
//! let packet: ServerPacket = protocol.deserialize_server(&bytes)?;
//!
//! // To deserialize a packet from a client
//! let packet: ClientPacket = protocol.deserialize_client(&bytes)?;
//! # }
//! ```
//!
//! The [`ProtocolSerializationExt`][3] trait is also provided to allow
//! for less typing when doing lots of serialization or deserialization.
//!
//! [0]: https://airma.sh
//! [1]: https://crates.io/crates/airmash-protocol-v5
//! [2]: trait.Protocol.html
//! [3]: trait.ProtocolSerializationExt.html

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate server_v2_macros;

// #[macro_use]
// extern crate dimensioned;
#[macro_use]
extern crate derive_more;

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", macro_use)]
extern crate serde;

extern crate self as airmash_protocol;

#[macro_use]
mod detail;

pub mod consts;
mod enums;
mod packets;
mod traits;
mod types;

mod client_packet;
mod server_packet;

#[cfg(feature = "serde")]
pub mod custom;
pub mod error;

pub use self::client_packet::*;
pub use self::enums::*;
pub use self::packets::*;
pub use self::server_packet::*;
pub use self::traits::*;
pub use self::types::*;

// Namespace for really common types in error messages
// so that the error messages end up being shorter.
mod d {
    #[derive(Copy, Clone, Default)]
    pub struct BaseUnit<
        T,
        const LENGTH: isize,
        const TIME: isize,
        const HEALTH: isize,
        const ENERGY: isize,
        const ROT: isize,
    > {
        pub value: T,
    }

    /// A 2D Vector that works with unit conversions.
    ///
    /// **Note:** [`Position`][0], [`Velocity`][1],
    /// and [`Accel`][2] are all instances of this struct
    /// with different units.
    ///
    /// [0]: type.Position.html
    /// [1]: type.Velocity.html
    /// [2]: type.Accel.html
    #[derive(Default, Clone, Copy, PartialEq, Debug)]
    pub struct Vector2<T> {
        pub x: T,
        pub y: T,
    }
}
