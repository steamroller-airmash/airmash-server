use error::*;
use protocol_common::*;
use serde::*;

pub fn serialize(v: &Option<Player>, ser: &mut Serializer) -> Result<(), SerializeError> {
	v.unwrap_or(Player(0)).serialize(ser)
}
pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Option<Player>, DeserializeError> {
	let player = Player::deserialize(de)?;

	Ok(if player == Player(0) {
		None
	} else {
		Some(player)
	})
}
