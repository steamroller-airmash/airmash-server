
use protocol::serde_am::*;
use protocol::error::Error;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(features="serde", derive(Serialize, Deserialize))]
pub enum MobType {
	None,
	PredatorMissile,
	GoliathMissile,
	MohawkRocket,
	Upgrade,
	TornadoSingleMissile,
	TornadoTripleMissile,
	ProwlerMissile,
	Shield,
	Inferno
}

impl MobType {
	pub fn to_u8(&self) -> u8 {
		match *self {
			MobType::None => 0,
			MobType::PredatorMissile => 1,
			MobType::GoliathMissile => 2,
			MobType::MohawkRocket => 3,
			MobType::Upgrade => 4,
			MobType::TornadoSingleMissile => 5,
			MobType::TornadoTripleMissile => 6,
			MobType::ProwlerMissile => 7,
			MobType::Shield => 8,
			MobType::Inferno => 9
		}
	}
	pub fn from_u8(v: u8) -> Option<Self> {
		Some(match v {
			0 => MobType::None,
			1 => MobType::PredatorMissile,
			2 => MobType::GoliathMissile,
			3 => MobType::MohawkRocket,
			4 => MobType::Upgrade,
			5 => MobType::TornadoSingleMissile,
			6 => MobType::TornadoTripleMissile,
			7 => MobType::ProwlerMissile,
			8 => MobType::Shield,
			9 => MobType::Inferno,
			_ => return None
		})
	}
}

impl Serialize for MobType {
	fn serialize(&self, ser: &mut Serializer) -> Result<()> {
		ser.serialize_u8(self.to_u8())
	}
}
impl<'de> Deserialize<'de> for MobType {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
		let ival = de.deserialize_u8()?;
		match Self::from_u8(ival) {
			Some(v) => Ok(v),
			None => Err(Error::InvalidMobType(ival))
		}
	}
}
