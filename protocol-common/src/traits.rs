use {ClientPacket, ServerPacket};

use std::error::Error;

type ServerPacketIterator = Box<Iterator<Item = Vec<u8>>>;

/// Interface to implement for all protocols.
pub trait Protocol: Sync + Send {
	type SerializeError: Error;
	type DeserializeError: Error;

	/// Unique version number for the protocol.
	fn version(&self) -> u8;

	fn serialize_client(
		&self,
		packet: &ClientPacket,
	) -> Result<ServerPacketIterator, Self::SerializeError>;
	fn serialize_server(
		&self,
		packet: &ServerPacket,
	) -> Result<ServerPacketIterator, Self::SerializeError>;

	fn deserialize_client(&self, data: &[u8]) -> Result<ClientPacket, Self::DeserializeError>;
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
	fn serialize<U>(&self, packet: U) -> Result<ServerPacketIterator, Self::SerializeError>
	where
		U: Into<T>;

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
