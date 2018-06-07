
use protocol::serde_am::*;

use bit_field::BitField;

/// Key state bitfield for PlayerUpdate packet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
#[cfg_attr(features="serde", derive(Serialize, Deserialize))]
pub struct ServerKeyState(pub u8);

impl ServerKeyState {
	pub const UP:        usize = 0;
	pub const DOWN:      usize = 1;
	pub const LEFT:      usize = 2;
	pub const RIGHT:     usize = 3;
	pub const BOOST:     usize = 4;
	pub const STRAFE:    usize = 5;
	pub const STEALTH:   usize = 6;
	pub const FLAGSPEED: usize = 7;

	pub fn up(&self) -> bool {
		self.0.get_bit(Self::UP)
	}
	pub fn down(&self) -> bool {
		self.0.get_bit(Self::DOWN)
	}
	pub fn left(&self) -> bool {
		self.0.get_bit(Self::LEFT)
	}
	pub fn right(&self) -> bool {
		self.0.get_bit(Self::RIGHT)
	}
	pub fn boost(&self) -> bool {
		self.0.get_bit(Self::BOOST)
	}
	pub fn strafe(&self) -> bool {
		self.0.get_bit(Self::STRAFE)
	}
	pub fn stealthed(&self) -> bool {
		self.0.get_bit(Self::STEALTH)
	}
	pub fn flagspeed(&self) -> bool {
		self.0.get_bit(Self::FLAGSPEED)
	}

	pub fn set(&mut self, bit: usize, state: bool) {
		self.0.set_bit(bit, state);
	}
	pub fn get(&mut self, bit: usize) -> bool {
		self.0.get_bit(bit)
	}
}

impl Serialize for ServerKeyState {
	fn serialize(&self, ser: &mut Serializer) -> Result<()> {
		ser.serialize_u8(self.0)
	}
}
impl<'de> Deserialize<'de> for ServerKeyState {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
		Ok(ServerKeyState(de.deserialize_u8()?))
	}
}
