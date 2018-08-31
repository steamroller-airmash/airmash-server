use {ClientPacket, ServerPacket};

use std::fmt::Display;

pub trait Protocol: Sync + Send {
	type SerializeError: Display;
	type DeserializeError: Display;

	fn version(&self) -> u8;

	fn serialize_client(&self, packet: &ClientPacket) -> Result<Vec<u8>, Self::SerializeError>;
	fn serialize_server(&self, packet: &ServerPacket) -> Result<Vec<u8>, Self::SerializeError>;

	fn deserialize_client(&self, data: &[u8]) -> Result<ClientPacket, Self::DeserializeError>;
	fn deserialize_server(&self, data: &[u8]) -> Result<ServerPacket, Self::DeserializeError>;
}

pub trait ProtocolSerializationExt<T>: Protocol {
	fn serialize<U>(&self, packet: U) -> Result<Vec<u8>, Self::SerializeError>
	where
		U: Into<T>;

	fn deserialize(&self, data: &[u8]) -> Result<T, Self::DeserializeError>;
}

impl<T> ProtocolSerializationExt<ServerPacket> for T
where
	T: Protocol + Sync + Send,
{
	fn serialize<U>(&self, packet: U) -> Result<Vec<u8>, Self::SerializeError>
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
	fn serialize<U>(&self, packet: U) -> Result<Vec<u8>, Self::SerializeError>
	where
		U: Into<ClientPacket>,
	{
		self.serialize_client(&packet.into())
	}

	fn deserialize(&self, data: &[u8]) -> Result<ClientPacket, Self::DeserializeError> {
		self.deserialize_client(data)
	}
}
