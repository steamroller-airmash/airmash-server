use error::*;
use protocol_common::*;
use serde::*;

const SHIFT: i32 = 32768;
const MULT: BaseType = 32768.0;

fn serialize_scalar(v: AccelScalar, ser: &mut Serializer) -> Result<(), SerializeError> {
	(((v.inner() * MULT) as i32 + SHIFT) as u16).serialize(ser)
}
fn deserialize_scalar<'de>(de: &mut Deserializer<'de>) -> Result<AccelScalar, DeserializeError> {
	Ok(((((u16::deserialize(de)? as i32) - SHIFT) as BaseType) / MULT).into())
}

pub fn serialize(v: &Accel, ser: &mut Serializer) -> Result<(), SerializeError> {
	serialize_scalar(v.x, ser)?;
	serialize_scalar(v.y, ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Accel, DeserializeError> {
	Ok(Accel::new(deserialize_scalar(de)?, deserialize_scalar(de)?))
}
