use error::*;
use protocol_common::{BaseType, Distance};
use serde::*;

const SHIFT: i32 = 32768;
const MULT: BaseType = 4.0;

pub fn serialize(val: &Distance, ser: &mut Serializer) -> Result<(), SerializeError> {
	ser.serialize_u16(((val.inner() * MULT) as i32 + SHIFT) as u16)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Distance, DeserializeError> {
	Ok(((((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT).into())
}
