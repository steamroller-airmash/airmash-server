use error::*;
use serde::*;

pub fn serialize(val: u32, ser: &mut Serializer) -> Result<(), SerializeError> {
	((val >> 8) as u16).serialize(ser)?;
	(val as u8).serialize(ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<u32, DeserializeError> {
	let hi = u16::deserialize(de)? as u32;
	let lo = u8::deserialize(de)? as u32;

	Ok((hi << 8) | lo)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn roundtrip_4325381() {
		let val = (1 << 22) + (1 << 15) + 5;
		let mut ser = Serializer::new();
		serialize(val, &mut ser).unwrap();
		let mut de = Deserializer::new(&ser.bytes);
		let new = deserialize(&mut de).unwrap();

		assert_eq!(val, new);
	}

	#[test]
	fn serialize_4325381() {
		let val = (1 << 22) + (1 << 15) + 5;
		let mut ser = Serializer::new();
		serialize(val, &mut ser).unwrap();

		let buf = ser.bytes;
		assert_eq!(buf[0], 128);
		assert_eq!(buf[1], 64);
		assert_eq!(buf[2], 5);
	}
}
