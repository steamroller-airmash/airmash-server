use error::*;
use protocol_common::*;
use serde::*;

const MULT: BaseType = 6553.6;

pub fn serialize(rot: &Rotation, ser: &mut Serializer) -> Result<(), SerializeError> {
	((rot.inner() * MULT) as u16).serialize(ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Rotation, DeserializeError> {
	Ok(Rotation::new(u16::deserialize(de)? as f32) / MULT)
}
