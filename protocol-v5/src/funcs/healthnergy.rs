use error::*;
use protocol_common::*;
use serde::*;

const MULT: BaseType = 255.0;

pub fn serialize(val: BaseType, ser: &mut Serializer) -> Result<(), SerializeError> {
	ser.serialize_u8((val * MULT) as u8)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<BaseType, DeserializeError> {
	Ok((de.deserialize_u8()? as BaseType) / MULT)
}
