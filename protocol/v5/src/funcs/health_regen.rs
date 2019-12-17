use super::*;
use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(val: &HealthRegen, ser: &mut Serializer) -> Result<(), SerializeError> {
	regen::serialize(val.inner(), ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<HealthRegen, DeserializeError> {
	regen::deserialize(de).map(|x| x.into())
}
