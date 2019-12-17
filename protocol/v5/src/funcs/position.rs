use super::{coordx, coordy};
use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(pos: &Position, ser: &mut Serializer) -> Result<(), SerializeError> {
	coordx::serialize(&pos.x, ser)?;
	coordy::serialize(&pos.y, ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeserializeError> {
	Ok(Position::new(
		coordx::deserialize(de)?,
		coordy::deserialize(de)?,
	))
}
