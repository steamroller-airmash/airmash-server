use super::array_small;
use error::*;
use serde::*;

use std::borrow::Cow;

pub fn serialize(s: &str, ser: &mut Serializer) -> Result<(), SerializeError> {
    array_small::serialize(s.as_bytes(), ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Cow<'static, str>, DeserializeError> {
    Ok(String::from_utf8(array_small::deserialize(de)?)?.into())
}
