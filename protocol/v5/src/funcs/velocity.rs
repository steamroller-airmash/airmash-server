use super::speed;
use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(v: &Velocity, ser: &mut Serializer) -> Result<(), SerializeError> {
	speed::serialize(&v.x, ser)?;
	speed::serialize(&v.y, ser)
}

pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Velocity, DeserializeError> {
	Ok(Velocity::new(
		speed::deserialize(de)?,
		speed::deserialize(de)?,
	))
}
