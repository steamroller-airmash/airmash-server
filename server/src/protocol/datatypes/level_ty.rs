
use protocol::error::Error;
use protocol::serde_am::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum PlayerLevelType {
	Login,
	LevelUp
}

impl Serialize for PlayerLevelType {
	fn serialize(&self, ser: &mut Serializer) -> Result<()> {
		let val = match self {
			PlayerLevelType::Login => 0,
			PlayerLevelType::LevelUp => 1
		};
		ser.serialize_u8(val)
	}
}
impl<'de> Deserialize<'de> for PlayerLevelType {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
		let val = de.deserialize_u8()?;
		match val {
			0 => Ok(PlayerLevelType::Login),
			1 => Ok(PlayerLevelType::LevelUp),
			_ => Err(Error::InvalidLevelType(val))
		}
	}
}
