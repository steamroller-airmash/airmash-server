
//! **THIS LIBRARY IS IN ALPHA!**
//! **USE AT YOUR OWN RISK**
//! 
//! This library contains definitions 
//! and serialization/deserialization methods
//! for the protocol for the game 
//! [`AIRMASH`](https://airma.sh). It implements
//! serialization and deserialization and is 
//! intended to be used for creating bots or 
//! servers using this protocol.
//! 
//! # Basic Usage
//! Serialization and deserialization of packets
//! is simple on the caller's side. All that
//! must be done is to call 
//! [`from_bytes`](fn.from_bytes.html) to
//! deserialize and to call
//! [`to_bytes`](fn.to_bytes.html) to serialize.
//! The [`ServerPacket`](server/struct.serverpacket.html)
//! and [`ClientPacket`](client/struct.clientpacket.html)
//! structs will take care of decoding the message 
//! type and message fields for you as well as 
//! encoding them. If an invalid message is passed 
//! then deserialize will return an error indicating
//! what went wrong. (Note: it will currently panic
//! in some cases. See TODOs). If an error occurse while
//! serializing, an error will be returned too. 
//! (Note: will panic currently. See TODOs).
//! 
//! # Client-Side
//! Clients using this library will be deserializing
//! packets into the 
//! [`ServerPacket`](server/enum.serverpacket.html)
//! enum and will be responding by serializing
//! [`ClientPacket`](client/enum.clientpacket.html)s
//! and sending those to the server.
//! 
//! # Server-Side
//! Servers using this library will be doing the 
//! opposite of clients. They will deserialize
//! [`ServerPacket`](server/enum.serverpacket.html)s
//! and will be serializing
//! [`ClientPacket`](client/enum.clientpacket.html)s.
//! 
//! # Use with Serde
//! All protocol types are able to be serialized and 
//! deserialized using serde. This is not enabled
//! by default, but can be enabled by turning on
//! the feature `"serde"`. Note that serde is not 
//! required for regular use of the library.
//! 


// # TODOs
// There is still a bunch of things that can be 
// improved within the library:
// 
// - Change teams to an enum. (Currently a `u8`)
// - Complete packet field documentation/figure out
//   what all packet fields actually do.
// - Complete documentation of BTR 
//   [`ServerCustom`](server/struct.servercustom.html)
//   data format.
// - Write unit tests for all serialization and deserialization
//   groups within [`field.rs`](../src/airmash_protocol/field.rs.html).
// - More internal documentation on specific protocol data types.
//   This should probably go within 
//   [`field.rs`](../src/airmash_protocol/field.rs.html) too.
// 

mod serde_am;

mod de;
mod error;
mod field;
mod ser;

mod impls;
mod packet_impls;
mod codes;
mod include;
mod packet;

mod datatypes;

pub mod server {
	pub use protocol::include::server::*;
	pub use protocol::packet::ServerPacket;
}
pub mod client {
	pub use protocol::include::client::*;
	pub use protocol::packet::ClientPacket;
}

pub use protocol::include::{
	PlayerStatus,
	PlaneType,
	KeyCode,
	MobType,
	CommandReplyType,
	UpgradeType,
	PowerupType,
	FlagUpdateType,
	FirewallUpdateType,
	FirewallStatus,
	LeaveHorizonType,
	ErrorType,
	ServerCustomType,
	ServerMessageType,
	PlayerLevelType,
	FlagCode
};

pub use self::ser::to_bytes;
pub use self::de::from_bytes;
pub use self::error::SerError as Error;
pub use self::error::DeError;

pub use self::client::ClientPacket;
pub use self::server::ServerPacket;

pub use self::datatypes::*;
