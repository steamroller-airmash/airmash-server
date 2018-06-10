
use protocol::serde_am::*;
use protocol::error::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(features="serde", derive(Serialize, Deseralize))]
pub enum PlayerStatus {
	Alive,
	Dead
}

impl PlayerStatus {
	pub fn to_u8(&self) -> u8 {
		match *self {
			PlayerStatus::Alive => 0,
			PlayerStatus::Dead => 1,
		}
	}
	pub fn from_u8(v: u8) -> Option<Self> {
		Some(match v {
			0 => PlayerStatus::Alive,
			1 => PlayerStatus::Dead,
			_ => return None
		})
	}
}

impl Serialize for PlayerStatus {
	fn serialize(&self, ser: &mut Serializer) -> Result<()> {
		ser.serialize_u8(self.to_u8())
	}
}
impl<'de> Deserialize<'de> for PlayerStatus {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
		let ival = de.deserialize_u8()?;
		match Self::from_u8(ival) {
			Some(v) => Ok(v),
			None => Err(Error::InvalidPlayerStatus(ival))
		}
	}
}
