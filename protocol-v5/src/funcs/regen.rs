use error::*;
use protocol_common::BaseType;
use serde::*;

const SHIFT: i32 = 32768;
const MULT: BaseType = 1.0e6;

pub fn serialize(val: BaseType, ser: &mut Serializer) -> Result<(), SerializeError> {
	ser.serialize_u16(((val * MULT) as i32 + SHIFT) as u16)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<BaseType, DeserializeError> {
	Ok((((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT)
}
