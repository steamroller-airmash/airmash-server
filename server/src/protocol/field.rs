use protocol::serde_am::*;
use protocol::error::SerError as Error;

pub type SerResult = Result<(), SerError>;

pub mod textbig {
	use protocol::field::*;

	pub fn serialize(val: &str, ser: &mut Serializer) -> SerResult {
		let bytes = val.as_bytes();

		if bytes.len() > 0xFFFF {
			return Err(Error::ArrayLengthTooBig);
		}

		println!("{}", bytes.len());
		ser.serialize_u16(bytes.len() as u16)?;
		ser.serialize_bytes(bytes)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<String, DeError> {
		let len = de.deserialize_u16()?;
		Ok(de.deserialize_str(len as usize)?.to_string())
	}
}

pub mod text {
	use protocol::field::*;

	pub fn serialize(val: &str, ser: &mut Serializer) -> SerResult {
		let bytes = val.as_bytes();

		if bytes.len() > 0xFF {
			return Err(Error::ArrayLengthTooBig);
		}

		ser.serialize_u8(bytes.len() as u8)?;
		ser.serialize_bytes(bytes)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<String, DeError> {
		let len = de.deserialize_u8()?;
		Ok(de.deserialize_str(len as usize)?.to_string())
	}
}

pub mod array {
	use protocol::field::*;
	use std::vec::Vec;

	pub fn serialize<T>(arr: &Vec<T>, ser: &mut Serializer) -> SerResult
	where
		T: Serialize,
	{
		if arr.len() > 0xFFFF {
			return Err(Error::ArrayLengthTooBig);
		}

		let s = ser.serialize_u16(arr.len() as u16)?;

		for val in arr {
			val.serialize(ser)?;
		}

		Ok(s)
	}
	pub fn deserialize<'de, T>(de: &mut Deserializer<'de>) -> Result<Vec<T>, DeError>
	where
		T: Deserialize<'de>,
	{
		let len = de.deserialize_u16()?;
		let mut vec = vec![];

		for _ in 0..len {
			vec.push(T::deserialize(de)?);
		}

		Ok(vec)
	}
}

pub mod arraysmall {
	use protocol::field::*;

	pub fn serialize<T>(arr: &[T], ser: &mut Serializer) -> SerResult
	where
		T: Serialize,
	{
		if arr.len() > 0xFF {
			return Err(Error::ArrayLengthTooBig);
		}

		let s = ser.serialize_u8(arr.len() as u8)?;

		for val in arr {
			val.serialize(ser)?;
		}

		Ok(s)
	}
	pub fn deserialize<'de, T>(de: &mut Deserializer<'de>) -> Result<Vec<T>, DeError>
	where
		T: Deserialize<'de>,
	{
		let len = de.deserialize_u8()?;
		let mut vec = vec![];

		for _ in 0..len {
			vec.push(T::deserialize(de)?);
		}

		Ok(vec)
	}
}

pub mod rotation {
	use protocol::field::*;
	use types::*;

	const MULT: f32 = 6553.6;

	pub fn serialize(val: &Rotation, ser: &mut Serializer) -> SerResult {
		ser.serialize_u16((val.inner() * MULT) as u16)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Rotation, DeError> {
		Ok(Rotation::new(de.deserialize_u16()? as f32) / MULT)
	}
}

pub mod healthnergy {
	use protocol::field::*;

	const MULT: f32 = 255.0;

	pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult {
		ser.serialize_u8((*val * MULT) as u8)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32, DeError> {
		Ok((de.deserialize_u8()? as f32) / MULT)
	}
}

pub mod uint24 {
	use protocol::field::*;

	pub fn serialize(val: u32, ser: &mut Serializer) -> SerResult {
		ser.serialize_u16((val >> 8) as u16)?;
		ser.serialize_u8(val as u8)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<u32, DeError> {
		let hi = de.deserialize_u16()?;
		let lo = de.deserialize_u8()?;

		Ok(((hi << 8) as u32) | (lo as u32))
	}
}

pub mod coord24 {
	use protocol::field::*;

	// Note: This assumes that f32 has enough precision,
	//       the client uses f64 as it is written in js

	const SHIFT: i32 = 8388608;
	const MULT: f32 = 512.0;

	pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult  {
		uint24::serialize(((*val * MULT) as i32 + SHIFT) as u32, ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32, DeError> {
		Ok((((uint24::deserialize(de)? as i32) - SHIFT) as f32) / MULT)
	}
}

pub mod accel {
	use protocol::field::*;
	use types::*;

	// Note: This assumes that f32 has enough precision,
	//       the client uses f64 as it is written in js

	const SHIFT: i32 = 32768;
	const MULT: f32 = 32768.0;

	pub fn serialize(val: &Accel, ser: &mut Serializer) -> SerResult {
		ser.serialize_u16(((val.x.inner() * MULT) as i32 + SHIFT) as u16)?;
		ser.serialize_u16(((val.y.inner() * MULT) as i32 + SHIFT) as u16)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Accel, DeError> {
		let x: f32 = (((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT;
		let y: f32 = (((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT;

		Ok(Accel::new(
			AccelScalar::new(x),
			AccelScalar::new(y)
		))
	}
}

pub mod velocity {
	use protocol::field::*;
	use types::*;

	// Note: This assumes that f32 has enough precision,
	//       the client uses f64 as it is written in js

	const SHIFT: i32 = 32768;
	const MULT: f32 = 1638.4;

	pub fn serialize(val: &Velocity, ser: &mut Serializer) -> SerResult {
		ser.serialize_u16(((val.x.inner() * MULT) as i32 + SHIFT) as u16)?;
		ser.serialize_u16(((val.y.inner() * MULT) as i32 + SHIFT) as u16)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Velocity, DeError> {
		let x: f32 = (((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT;
		let y: f32 = (((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT;

		Ok(Velocity::new(
			Speed::new(x),
			Speed::new(y)
		))
	}
}

pub mod speed {
	use protocol::field::*;
	use types::Speed;

	// Note: This assumes that f32 has enough precision,
	//       the client uses f64 as it is written in js

	const SHIFT: i32 = 32768;
	const MULT: f32 = 1638.4;

	pub fn serialize(val: &Speed, ser: &mut Serializer) -> SerResult {
		ser.serialize_u16(((val.inner() * MULT) as i32 + SHIFT) as u16)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Speed, DeError> {
		Ok(Speed::new((((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT))
	}
}

macro_rules! shift_mult_decode {
	($name:ident, $shift:expr, $mult:expr) => {
		pub mod $name {
			use protocol::field::*;

			// Note: This assumes that f32 has enough precision,
			//       the client uses f64 as it is written in js

			const SHIFT: i32 = $shift;
			const MULT: f32 = $mult;

			pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult {
				ser.serialize_u16(((*val * MULT) as i32 + SHIFT) as u16)
			}
			pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32, DeError> {
				Ok((((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT)
			}
		}
	};
}

shift_mult_decode!(coordy, 32768, 4.0);
shift_mult_decode!(coordx, 32768, 2.0);
shift_mult_decode!(regen, 32768, 1.0e6);

pub mod entity {
	use protocol::field::*;
	use specs::Entity;

	pub fn serialize(val: &Entity, ser: &mut Serializer) -> SerResult {
		assert!(val.id() < 0xFFFF);
		ser.serialize_u16(val.id() as u16)
	}
	pub fn deserialize<'de>(_: &mut Deserializer<'de>) -> Result<Entity, DeError> {
		Err(DeError::EntityMayNotBeDeserialized)
	}
}

pub mod option_entity {
	use protocol::field::*;
	use specs::Entity;

	pub fn serialize(val: &Option<Entity>, ser: &mut Serializer) -> SerResult {
		match val {
			Some(val) => {
				assert!(val.id() < 0xFFFF);
				ser.serialize_u16(val.id() as u16)
			},
			None => {
				ser.serialize_u16(0)
			}
		}
	}
	pub fn deserialize<'de>(_: &mut Deserializer<'de>) -> Result<Option<Entity>, DeError> {
		Err(DeError::EntityMayNotBeDeserialized)
	}
}

macro_rules! serde_inner {
	($name:ident, $type:ident) => {
		pub mod $name {
			use protocol::field::*;
			use types::*;
			
			pub fn deserialize_inner<'de, T>(de: &mut Deserializer<'de>) -> Result<T, DeError>
				where T: Deserialize<'de>
			{
				T::deserialize(de)
			}

			pub fn serialize(val: &$type, ser: &mut Serializer) -> SerResult {
				val.0.serialize(ser)
			}
			pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<$type, DeError> {
				Ok($type(deserialize_inner(de)?))
			}
		}
	}
}

serde_inner!(score, Score);
serde_inner!(level, Level);
serde_inner!(team, Team);

pub mod pos {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Position, ser: &mut Serializer) -> SerResult {
		coordx::serialize(&val.x.inner(), ser)?;
		coordy::serialize(&val.y.inner(), ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeError> {
		Ok(Position::new(
			Distance::new(coordx::deserialize(de)?),
			Distance::new(coordy::deserialize(de)?)
		))
	}
}

pub mod pos24 {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Position, ser: &mut Serializer) -> SerResult {
		coord24::serialize(&val.x.inner(), ser)?;
		coord24::serialize(&val.y.inner(), ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeError> {
		Ok(Position::new(
			Distance::new(coord24::deserialize(de)?),
			Distance::new(coord24::deserialize(de)?)
		))
	}
}

pub mod pos_f32 {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Position, ser: &mut Serializer) -> SerResult {
		ser.serialize_f32(val.x.inner())?;
		ser.serialize_f32(val.y.inner())
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeError> {
		Ok(Position::new(
			Distance::new(de.deserialize_f32()?),
			Distance::new(de.deserialize_f32()?)
		))
	}
}

pub mod vel_u {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Velocity, ser: &mut Serializer) -> SerResult {
		speed::serialize(&val.x, ser)?;
		speed::serialize(&val.y, ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Velocity, DeError> {
		Ok(Velocity::new(
			speed::deserialize(de)?,
			speed::deserialize(de)?
		))
	}
}

pub mod lowrespos {
	use protocol::field::*;
	use types::*;

	pub fn serialize(pos: &Position, ser: &mut Serializer) -> SerResult {
		ser.serialize_u8(((pos.x.inner() / 128.0) as i32 + 128) as u8)?;
		ser.serialize_u8(((pos.y.inner() / 128.0) as i32 + 128) as u8)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Position, DeError> {
		Ok(Position::new(
			Distance::new(((de.deserialize_u8()? as i32 - 128) * 128) as f32),
			Distance::new(((de.deserialize_u8()? as i32 - 128) * 64) as f32)
		))
	}
}

pub mod health {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Health, ser: &mut Serializer) -> SerResult {
		healthnergy::serialize(&val.inner(), ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Health, DeError> {
		Ok(Health::new(healthnergy::deserialize(de)?))
	}
}

pub mod energy {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Energy, ser: &mut Serializer) -> SerResult {
		healthnergy::serialize(&val.inner(), ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Energy, DeError> {
		Ok(Energy::new(healthnergy::deserialize(de)?))
	}
}

pub mod health_regen {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &HealthRegen, ser: &mut Serializer) -> SerResult {
		regen::serialize(&val.inner(), ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<HealthRegen, DeError> {
		Ok(HealthRegen::new(regen::deserialize(de)?))
	}
}

pub mod energy_regen {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &EnergyRegen, ser: &mut Serializer) -> SerResult {
		regen::serialize(&val.inner(), ser)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<EnergyRegen, DeError> {
		Ok(EnergyRegen::new(regen::deserialize(de)?))
	}
}

pub mod flag {
	use protocol::field::*;
	use types::*;

	pub fn serialize(val: &Team, ser: &mut Serializer) -> SerResult {
		assert!(val.0 < 0xFF);

		ser.serialize_u8(val.0 as u8)
	}
	pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Team, DeError> {
		Ok(Team(de.deserialize_u8()? as u16))
	}
}

