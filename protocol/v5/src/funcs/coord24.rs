use super::uint24;
use error::*;
use protocol_common::*;
use serde::*;

const SHIFT: i32 = 8388608;
const MULT: BaseType = 512.0;

pub fn serialize(val: &Distance, ser: &mut Serializer) -> Result<(), SerializeError> {
	uint24::serialize(((val.inner() * MULT) as i32 + SHIFT) as u32, ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Distance, DeserializeError> {
	Ok(((((uint24::deserialize(de)? as i32) - SHIFT) as f32) / MULT).into())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn roundtrip_0() {
		let pos = Distance::new(0.0);
		let mut ser = Serializer::new();
		serialize(&pos, &mut ser).unwrap();
		let mut de = Deserializer::new(&mut ser.bytes);
		let new = deserialize(&mut de).unwrap();

		eprintln!("new: {}\npos: {}", new, pos);

		assert!(new == pos);
	}
}
