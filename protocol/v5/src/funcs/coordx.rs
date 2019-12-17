use error::*;
use protocol_common::{BaseType, Distance};
use serde::*;

const SHIFT: i32 = 32768;
const MULT: BaseType = 2.0;

pub fn serialize(val: &Distance, ser: &mut Serializer) -> Result<(), SerializeError> {
	ser.serialize_u16(((val.inner() * MULT) as i32 + SHIFT) as u16)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Distance, DeserializeError> {
	Ok(((((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT).into())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn serialize_0() {
		let mut ser = Serializer::new();
		let val = Distance::new(0.0);
		serialize(&val, &mut ser).unwrap();

		let buf = ser.bytes;
		assert!(buf[0] == 0 && buf[1] == 128);
	}

	#[test]
	fn deserialize_0() {
		let buf = [0, 128];
		let mut de = Deserializer::new(&buf);
		let val = deserialize(&mut de).unwrap();

		assert!(val == 0.0.into());
	}
}
