use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(pos: &Position, ser: &mut Serializer) -> Result<(), SerializeError> {
	ser.serialize_f32(pos.x.inner())?;
	ser.serialize_f32(pos.y.inner())
}

pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeserializeError> {
	Ok(Position::new(de.deserialize_f32()?, de.deserialize_f32()?))
}
