use super::coord24;
use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(pos: &Position, ser: &mut Serializer) -> Result<(), SerializeError> {
	coord24::serialize(&pos.x, ser)?;
	coord24::serialize(&pos.y, ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeserializeError> {
	Ok(Position::new(
		coord24::deserialize(de)?,
		coord24::deserialize(de)?,
	))
}
