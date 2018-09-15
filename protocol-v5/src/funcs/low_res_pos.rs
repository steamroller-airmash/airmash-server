use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(pos: &Option<Position>, ser: &mut Serializer) -> Result<(), SerializeError> {
	let x = pos
		.map(|v| ((v.x.inner() / 128.0) as i32 + 128) as u8)
		.unwrap_or(0);
	let y = pos
		.map(|v| ((v.y.inner() / 128.0) as i32 + 128) as u8)
		.unwrap_or(0);

	(x, y).serialize(ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Option<Position>, DeserializeError> {
	let (x, y) = <(u8, u8)>::deserialize(de)?;

	if x == 0 && y == 0 {
		return Ok(None);
	}

	Ok(Position::new(
		((x as i32 - 128) * 128) as f32,
		((y as i32 - 128) * 128) as f32,
	).into())
}
