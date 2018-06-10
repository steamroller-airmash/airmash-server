
use protocol::serde_am::*;

use bit_field::BitField;

/// Key state bitfield for PlayerUpdate packet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
#[cfg_attr(features="serde", derive(Serialize, Deserialize))]
pub struct ServerKeyState {
	pub up : bool,
	pub down : bool,
	pub left : bool,
	pub right : bool,
	pub boost : bool,
	pub strafe : bool,
	pub stealth : bool,
	pub flagspeed : bool,
}

impl Serialize for ServerKeyState {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError> {
		let val = 0
			| (self.up as u8)        << 0
			| (self.down as u8)      << 1
			| (self.left as u8)      << 2
			| (self.right as u8)     << 3
			| (self.boost as u8)     << 4
			| (self.strafe as u8)    << 5
			| (self.stealth as u8)   << 6
			| (self.flagspeed as u8) << 7;

		ser.serialize_u8(val)
	}
}
impl<'de> Deserialize<'de> for ServerKeyState {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self, DeError> {
		let val = de.deserialize_u8()?;

		Ok(ServerKeyState {
			up:       (val & 0b00000001) != 0,
			down:     (val & 0b00000010) != 0,
			left:     (val & 0b00000100) != 0,
			right:    (val & 0b00001000) != 0,
			boost:    (val & 0b00010000) != 0,
			strafe:   (val & 0b00100000) != 0,
			stealth:  (val & 0b01000000) != 0,
			flagspeed:(val & 0b10000000) != 0
		})
	}
}
