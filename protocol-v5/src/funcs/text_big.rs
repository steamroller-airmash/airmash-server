use super::array_large;
use error::*;
use serde::*;

pub fn serialize(s: &String, ser: &mut Serializer) -> Result<(), SerializeError> {
	array_large::serialize(s.as_bytes(), ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<String, DeserializeError> {
	Ok(String::from_utf8(array_large::deserialize(de)?)?)
}
