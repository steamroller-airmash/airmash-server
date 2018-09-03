use protocol_common::{ClientPacket, Protocol, ServerPacket};

type SerValue = Box<Iterator<Item = Vec<u8>>>;

use error::*;

pub struct ProtocolV5;

impl Protocol for ProtocolV5 {
	type SerializeError = SerializeError;
	type DeserializeError = DeserializeError;

	fn version(&self) -> u8 {
		5
	}

	fn serialize_client(&self, packet: &ClientPacket) -> Result<SerValue, SerializeError> {
		unimplemented!()
	}
	fn serialize_server(&self, packet: &ServerPacket) -> Result<SerValue, SerializeError> {
		unimplemented!()
	}

	fn deserialize_client(&self, bytes: &[u8]) -> Result<ClientPacket, DeserializeError> {
		unimplemented!()
	}
	fn deserialize_server(&self, bytes: &[u8]) -> Result<ServerPacket, DeserializeError> {
		unimplemented!()
	}
}
