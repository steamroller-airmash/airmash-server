use crate::{ClientPacket, ServerPacket};

use std::error::Error;

type ServerPacketIterator = Box<dyn Iterator<Item = Vec<u8>>>;

/// Interface to implement for all protocols.
pub trait Protocol: Sync + Send {
	/// Error for when a packet fails to serialize.
	///
	/// This should be broad enough to be able to deal
	/// with failures for both [`ClientPacket`][0] and
	/// [`ServerPacket`][1].
	///
	/// [0]: enum.ClientPacket.html
	/// [1]: enum.ServerPacket.html
	type SerializeError: Error;
	/// Error for when a packet fails to deserialize.
	///
	/// This should be broad enough to be able to deal
	/// with failures for both [`ClientPacket`][0] and
	/// [`ServerPacket`][1].
	///
	/// [0]: enum.ClientPacket.html
	/// [1]: enum.ServerPacket.html
	type DeserializeError: Error;

	/// Unique version number for the protocol.
	fn version(&self) -> u8;

	/// Serialize a client packet into some number of
	/// binary packet bodies.
	///
	/// For most packets this should be a 1-to-1 deserialization
	/// but the iterator return type is there to allow for
	/// polyfilling of the packets behind the scenes if a
	/// protocol backend doesn't support them directly.
	/// This can be used (for example) to send multiple packets
	/// for when more than 255 missiles are reflected with
	/// the same goliath deflect (which is not supported by
	/// protocol-v5).
	///
	/// For users of this interface it will most likely be
	/// more convienient to call
	/// [`ProtocolSerializationExt::serialize()`][2] with a
	/// [`ClientPacket`][1] instead, since it provides a
	/// unified interface to serializing a [`ClientPacket`][1]
	/// and a [`ServerPacket`][0].
	///
	/// # Panics
	/// This method should never panic (based on the input),
	/// instead it should return an appropriate error within
	/// the SerializeError type.
	///
	/// [0]: struct.ServerPacket.html
	/// [1]: struct.ClientPacket.html
	/// [2]: trait.ProtocolSerializationExt.html#ty.serialize
	fn serialize_client(
		&self,
		packet: &ClientPacket,
	) -> Result<ServerPacketIterator, Self::SerializeError>;

	/// Serialize a server packet into some number of
	/// binary packet bodies.
	///
	/// For most packets this should be a 1-to-1 deserialization
	/// but the iterator return type is there to allow for
	/// polyfilling of the packets behind the scenes if a
	/// protocol backend doesn't support them directly.
	/// This can be used (for example) to send multiple packets
	/// for when more than 255 missiles are reflected with
	/// the same goliath deflect (which is not supported by
	/// protocol-v5).
	///
	/// For users of this interface it will most likely be
	/// more convienient to call
	/// [`ProtocolSerializationExt::serialize()`][2] with a
	/// [`ServerPacket`][0] instead, since it provides a
	/// unified interface to serializing a [`ServerPacket`][0]
	/// and a [`ClientPacket`][1].
	///
	/// # Panics
	/// This method should never panic (based on the input),
	/// instead it should return an appropriate error within
	/// the SerializeError type.
	///
	/// [0]: struct.ServerPacket.html
	/// [1]: struct.ClientPacket.html
	/// [2]: trait.ProtocolSerializationExt.html#ty.serialize
	fn serialize_server(
		&self,
		packet: &ServerPacket,
	) -> Result<ServerPacketIterator, Self::SerializeError>;

	/// Deserialize a binary packet into a client packet.
	fn deserialize_client(&self, data: &[u8]) -> Result<ClientPacket, Self::DeserializeError>;

	/// Deserialize a binary packet into a server packet.
	fn deserialize_server(&self, data: &[u8]) -> Result<ServerPacket, Self::DeserializeError>;
}

/// Helper trait to make working with protocols easier.
///
/// This allows for [`ServerPacket`][0] and
/// [`ClientPacket`][1] to be serialized and deserialized
/// without using the `serialize_*` or `deserialize_*`
/// methods within the [`Protocol`] trait. Instead they
/// can use [`serialize`][2] and [`deserialize`][3] and
/// let type inference deduce the correct type.
///
/// **This trait should not be implemented by client code.**
///
/// [0]: enum.ServerPacket.html
/// [1]: enum.ClientPacket.html
/// [2]: #tymethod.serialize
/// [3]: #tymethod.deserialize
pub trait ProtocolSerializationExt<T>: Protocol {
	/// Serialize a packet. This can be either a
	/// [`ClientPacket`][1] or a [`ServerPacket`][0]
	/// depending on the instantiation of this trait.
	///
	/// [0]: enum.ServerPacket.html
	/// [1]: enum.ClientPacket.html
	fn serialize<U>(&self, packet: U) -> Result<ServerPacketIterator, Self::SerializeError>
	where
		U: Into<T>;

	/// Deserialize a packet. This can be either a
	/// [`ClientPacket`][1] or a [`ServerPacket`][0]
	/// depending on the instantiation of this trait.
	///
	/// [0]: enum.ServerPacket.html
	/// [1]: enum.ClientPacket.html
	fn deserialize(&self, data: &[u8]) -> Result<T, Self::DeserializeError>;
}

impl<T> ProtocolSerializationExt<ServerPacket> for T
where
	T: Protocol + Sync + Send,
{
	fn serialize<U>(&self, packet: U) -> Result<ServerPacketIterator, Self::SerializeError>
	where
		U: Into<ServerPacket>,
	{
		self.serialize_server(&packet.into())
	}

	fn deserialize(&self, data: &[u8]) -> Result<ServerPacket, Self::DeserializeError> {
		self.deserialize_server(data)
	}
}

impl<T> ProtocolSerializationExt<ClientPacket> for T
where
	T: Protocol + Sync + Send,
{
	fn serialize<U>(&self, packet: U) -> Result<ServerPacketIterator, Self::SerializeError>
	where
		U: Into<ClientPacket>,
	{
		self.serialize_client(&packet.into())
	}

	fn deserialize(&self, data: &[u8]) -> Result<ClientPacket, Self::DeserializeError> {
		self.deserialize_client(data)
	}
}
