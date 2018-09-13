use super::uint24;
use error::*;
use protocol_common::*;
use serde::*;

const SHIFT: i32 = 8388608;
const MULT: BaseType = 512.0;

pub fn serialize(val: &Distance, ser: &mut Serializer) -> Result<(), SerializeError> {
	uint24::serialize(((val.inner() * MULT) as i32 + SHIFT) as u32, ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Distance, DeserializeError> {
	Ok(((((uint24::deserialize(de)? as i32) - SHIFT) as f32) / MULT).into())
}
