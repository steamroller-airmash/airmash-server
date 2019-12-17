use error::*;
use protocol_common::*;
use serde::*;

const SHIFT: i32 = 32768;
const MULT: f32 = 1638.4;

pub fn serialize(v: &Speed, ser: &mut Serializer) -> Result<(), SerializeError> {
	(((v.inner() * MULT) as i32 + SHIFT) as u16).serialize(ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Speed, DeserializeError> {
	Ok(((((u16::deserialize(de)? as i32) - SHIFT) as f32) / MULT).into())
}
