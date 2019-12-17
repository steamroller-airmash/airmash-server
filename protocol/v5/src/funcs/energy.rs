use super::*;
use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(val: &Energy, ser: &mut Serializer) -> Result<(), SerializeError> {
	healthnergy::serialize(val.inner(), ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Energy, DeserializeError> {
	healthnergy::deserialize(de).map(|x| x.into())
}
