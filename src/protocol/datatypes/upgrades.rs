
use protocol::serde_am::*;

use bit_field::BitField;

/// Key state bitfield for PlayerUpdate packet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
#[cfg_attr(features="serde", derive(Serialize, Deserialize))]
pub struct Upgrades(pub u8);

impl Upgrades {
	pub const SHIELD: usize = 3;
	pub const INFERNO: usize = 4;

	/// The number of speed upgrades
	pub fn speed(&self) -> u8 {
		return self.get(0) as u8
			+ ((self.get(0) as u8) << 1)
			+ ((self.get(0) as u8) << 2);
	}

	pub fn shield(&self) -> bool {
		self.get(Self::SHIELD)
	}
	pub fn inferno(&self) -> bool {
		self.get(Self::INFERNO)
	}

	pub fn get(&self, bit: usize) -> bool {
		self.0.get_bit(bit)
	}

	pub fn set(&mut self, bit: usize, state: bool) {
		self.0.set_bit(bit, state);
	}

	pub fn set_speed(&mut self, speed: u8) {
		self.set(0, (speed & 0b001) != 0);
		self.set(1, (speed & 0b010) != 0);
		self.set(2, (speed & 0b100) != 0);
	}
}

impl Serialize for Upgrades {
	fn serialize(&self, ser: &mut Serializer) -> Result<()> {
		ser.serialize_u8(self.0)
	}
}
impl<'de> Deserialize<'de> for Upgrades {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
		Ok(Upgrades(de.deserialize_u8()?))
	}
}
