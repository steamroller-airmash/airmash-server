use error::*;
use serde::*;

pub fn serialize(val: u32, ser: &mut Serializer) -> Result<(), SerializeError> {
	((val >> 8) as u16).serialize(ser)?;
	(val as u8).serialize(ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<u32, DeserializeError> {
	let hi = u16::deserialize(de)?;
	let lo = u8::deserialize(de)?;

	Ok(((hi << 8) as u32) | (lo as u32))
}
