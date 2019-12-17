use super::*;
use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(val: &EnergyRegen, ser: &mut Serializer) -> Result<(), SerializeError> {
	regen::serialize(val.inner(), ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<EnergyRegen, DeserializeError> {
	regen::deserialize(de).map(|x| x.into())
}
