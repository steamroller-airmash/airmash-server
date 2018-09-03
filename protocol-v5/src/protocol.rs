use error::*;
use protocol_common::{ClientPacket, Protocol, ServerPacket};
use serde::{Deserializer, Serializer};

use std::iter::once;

type SerValue = Box<Iterator<Item = Vec<u8>>>;

/// Serializer/Deserialize for the airmash protocol version 5.
#[derive(Copy, Clone, Debug, Default)]
pub struct ProtocolV5;

impl Protocol for ProtocolV5 {
	type SerializeError = SerializeError;
	type DeserializeError = DeserializeError;

	fn version(&self) -> u8 {
		5
	}

	fn serialize_client(&self, packet: &ClientPacket) -> Result<SerValue, SerializeError> {
		Ok(Box::new(once(Serializer::serialize(packet)?)))
	}
	fn serialize_server(&self, packet: &ServerPacket) -> Result<SerValue, SerializeError> {
		Ok(Box::new(once(Serializer::serialize(packet)?)))
	}

	fn deserialize_client(&self, bytes: &[u8]) -> Result<ClientPacket, DeserializeError> {
		Deserializer::deserialize(bytes)
	}
	fn deserialize_server(&self, bytes: &[u8]) -> Result<ServerPacket, DeserializeError> {
		Deserializer::deserialize(bytes)
	}
}
