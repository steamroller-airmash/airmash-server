
use protocol::serde_am::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
#[cfg_attr(features="serde", derive(Serialize, Deserialize))]
pub struct Upgrades {
	pub speed: u8,
	pub shield: bool,
	pub inferno: bool
}

impl Serialize for Upgrades {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError> {
		assert!(self.speed < 8);

		let val = 0
			| (self.speed & 7)
			| (self.shield as u8) << 3
			| (self.inferno as u8) << 4;

		ser.serialize_u8(val)
	}
}
impl<'de> Deserialize<'de> for Upgrades {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self, DeError> {
		let val = de.deserialize_u8()?;

		Ok(Self {
			speed: val & 7,
			shield: (val & 8) != 0,
			inferno: (val & 16) != 0
		})
	}
}
