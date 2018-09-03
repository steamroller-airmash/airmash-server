use error::*;
use serde::Deserialize;

use std::mem;

pub struct Deserializer<'de> {
	bytes: &'de [u8],
}

impl<'de> Deserializer<'de> {
	fn new(bytes: &'de [u8]) -> Self {
		Self { bytes }
	}

	pub fn deserialize<T: Deserialize>(bytes: &'de [u8]) -> Result<T, DeserializeError> {
		T::deserialize(&mut Self::new(bytes))
	}
}

impl<'de> Deserializer<'de> {
	pub fn deserialize_u8(&mut self) -> Result<u8, DeserializeError> {
		if self.bytes.len() == 0 {
			return Err(DeserializeError {
				ty: DeserializeErrorType::UnexpectedEndOfMessage,
				trace: vec![],
			});
		}

		let b = self.bytes[0];
		self.bytes = &self.bytes[1..];
		Ok(b)
	}
	pub fn deserialize_u16(&mut self) -> Result<u16, DeserializeError> {
		let lo = self.deserialize_u8()?;
		let hi = self.deserialize_u8()?;

		Ok(((hi as u16) << 8) | (lo as u16))
	}
	pub fn deserialize_u32(&mut self) -> Result<u32, DeserializeError> {
		let lo = self.deserialize_u16()?;
		let hi = self.deserialize_u16()?;

		Ok(((hi as u32) << 16) | (lo as u32))
	}

	pub fn deserialize_i8(&mut self) -> Result<i8, DeserializeError> {
		Ok(self.deserialize_u8()? as i8)
	}
	pub fn deserialize_i16(&mut self) -> Result<i16, DeserializeError> {
		Ok(self.deserialize_u16()? as i16)
	}
	pub fn deserialize_i32(&mut self) -> Result<i32, DeserializeError> {
		Ok(self.deserialize_u32()? as i32)
	}

	pub fn deserialize_f32(&mut self) -> Result<f32, DeserializeError> {
		// Perform a bitwise transmute of u32 to f32. This will probably fail
		// when the sender and receiver endians aren't the same.
		// TODO: Figure this out
		Ok(unsafe { mem::transmute::<u32, f32>(self.deserialize_u32()?) })
	}

	fn deserialize_seq<T>(&mut self, len: usize) -> Result<Vec<T>, DeserializeError>
	where
		T: Deserialize,
	{
		let mut v = Vec::with_capacity(len);

		for i in 0..len {
			v.push(T::deserialize(self).map_err(move |e| {
				e.chain(FieldSpec {
					field: FieldName::Index(i),
					ty: Some("<array>"),
				})
			})?);
		}

		Ok(v)
	}

	pub fn deserialize_array_small<T: Deserialize>(&mut self) -> Result<Vec<T>, DeserializeError> {
		let len = self.deserialize_u8()? as usize;
		self.deserialize_seq(len)
	}

	pub fn deserialize_array_large<T: Deserialize>(&mut self) -> Result<Vec<T>, DeserializeError> {
		let len = self.deserialize_u16()? as usize;
		self.deserialize_seq(len)
	}
}

impl Deserialize for u8 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_u8()
	}
}
impl Deserialize for u16 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_u16()
	}
}
impl Deserialize for u32 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_u32()
	}
}

impl Deserialize for i8 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_i8()
	}
}
impl Deserialize for i16 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_i16()
	}
}
impl Deserialize for i32 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_i32()
	}
}

impl Deserialize for f32 {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		de.deserialize_f32()
	}
}

impl Deserialize for bool {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		Ok(if de.deserialize_u8()? == 0 {
			false
		} else {
			true
		})
	}
}

impl<T, U> Deserialize for (T, U)
where
	T: Deserialize,
	U: Deserialize,
{
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		Ok((T::deserialize(de)?, U::deserialize(de)?))
	}
}
