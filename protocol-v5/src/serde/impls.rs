use error::*;
use protocol_common::*;
use serde::*;

macro_rules! newtype_serde_decl {
	($ty:ident) => {
		impl Serialize for $ty {
			fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
				self.0.serialize(ser)
			}
		}

		impl Deserialize for $ty {
			fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
				Ok($ty(Deserialize::deserialize(de)?))
			}
		}
	};
}

macro_rules! enum_serde_decl {
	{
		$(
			$name:ident: $base:ident
		),*
	} => {
		$(
			impl Serialize for $name {
				fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
					$base::serialize(&(*self).into(), ser)
				}
			}

			impl Deserialize for $name {
				fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
					use std::convert::TryInto;

					Ok($base::deserialize(de)?.try_into()?)
				}
			}
		)*
	}
}

enum_serde_decl! {
	CommandReplyType: u8,
	ErrorType: u8,
	FirewallStatus: u8,
	FirewallUpdateType: u8,
	FlagCode: u16,
	FlagUpdateType: u8,
	GameType: u8,
	KeyCode: u8,
	LeaveHorizonType: u8,
	MobType: u8,
	PlaneType: u8,
	PlayerLevelType: u8,
	PlayerStatus: u8,
	PowerupType: u8,
	ServerCustomType: u8,
	ServerMessageType: u8,
	UpgradeType: u8
}

newtype_serde_decl!(Player);
newtype_serde_decl!(Team);
newtype_serde_decl!(Mob);
newtype_serde_decl!(Level);
newtype_serde_decl!(Score);

impl Serialize for Flag {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		if (self.0).0 > 0xFF {
			return Err(SerializeError {
				ty: SerializeErrorType::InvalidFlagId((self.0).0),
				trace: vec![],
			});
		}

		((self.0).0 as u8).serialize(ser)
	}
}

impl Deserialize for Flag {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		Ok(Flag(Team(u8::deserialize(de)? as u16)))
	}
}

impl Serialize for Upgrades {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		let val: u8 = (self.speed & 7) | ((self.shield as u8) << 3) | ((self.inferno as u8) << 4);

		val.serialize(ser)
	}
}

impl Deserialize for Upgrades {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		let val = u8::deserialize(de)?;

		Ok(Upgrades {
			speed: val & 7,
			shield: (val & (1 << 3)) != 0,
			inferno: (val & (1 << 4)) != 0,
		})
	}
}

impl Serialize for ServerKeyState {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		let val = 0
			| (self.up as u8) << 0
			| (self.down as u8) << 1
			| (self.left as u8) << 2
			| (self.right as u8) << 3
			| (self.boost as u8) << 4
			| (self.strafe as u8) << 5
			| (self.stealth as u8) << 6
			| (self.flagspeed as u8) << 7;

		ser.serialize_u8(val)
	}
}

impl Deserialize for ServerKeyState {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		let val = de.deserialize_u8()?;

		Ok(ServerKeyState {
			up: (val & 0b00000001) != 0,
			down: (val & 0b00000010) != 0,
			left: (val & 0b00000100) != 0,
			right: (val & 0b00001000) != 0,
			boost: (val & 0b00010000) != 0,
			strafe: (val & 0b00100000) != 0,
			stealth: (val & 0b01000000) != 0,
			flagspeed: (val & 0b10000000) != 0,
		})
	}
}
