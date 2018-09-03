use std::mem;

use error::*;
use serde::Serialize;

pub struct Serializer {
	bytes: Vec<u8>,
}

impl Serializer {
	fn new() -> Self {
		Self { bytes: vec![] }
	}

	pub fn serialize<T: Serialize>(v: &T) -> Result<Vec<u8>, SerializeError> {
		let mut me = Self::new();

		v.serialize(&mut me)?;

		Ok(me.bytes)
	}
}

impl Serializer {
	pub fn serialize_u8(&mut self, v: u8) -> Result<(), SerializeError> {
		self.bytes.push(v);
		Ok(())
	}
	pub fn serialize_u16(&mut self, v: u16) -> Result<(), SerializeError> {
		self.serialize_u8(v as u8)?;
		self.serialize_u8((v >> 8) as u8)
	}
	pub fn serialize_u32(&mut self, v: u32) -> Result<(), SerializeError> {
		self.serialize_u16(v as u16)?;
		self.serialize_u16((v >> 16) as u16)
	}

	pub fn serialize_i8(&mut self, v: i8) -> Result<(), SerializeError> {
		self.serialize_u8(v as u8)
	}
	pub fn serialize_i16(&mut self, v: i16) -> Result<(), SerializeError> {
		self.serialize_u16(v as u16)
	}
	pub fn serialize_i32(&mut self, v: i32) -> Result<(), SerializeError> {
		self.serialize_u32(v as u32)
	}

	pub fn serialize_f32(&mut self, v: f32) -> Result<(), SerializeError> {
		// Use transmute here to provide a bitwise copy.
		// This assumes that floats are encoded with IEEE-754
		// encoding with the same endian as u32. (So it'll probably
		// break on a big-endian platform)
		self.serialize_u32(unsafe { mem::transmute::<f32, u32>(v) })
	}

	fn serialize_seq<T: Serialize>(&mut self, seq: &[T]) -> Result<(), SerializeError> {
		for (i, item) in seq.iter().enumerate() {
			item.serialize(self).map_err(move |e| {
				e.chain(FieldSpec {
					field: FieldName::Index(i),
					ty: Some("<array>"),
				})
			})?;
		}

		Ok(())
	}

	pub fn serialize_array_small<T: Serialize>(&mut self, seq: &[T]) -> Result<(), SerializeError> {
		let len = seq.len();

		if len > 0xFF {
			return Err(SerializeError {
				ty: SerializeErrorType::ArrayTooLarge(0xFF),
				trace: vec![],
			});
		}

		self.serialize_u8(len as u8)?;
		self.serialize_seq(seq)
	}
	pub fn serialize_array_large<T: Serialize>(&mut self, seq: &[T]) -> Result<(), SerializeError> {
		let len = seq.len();

		if len > 0xFFFF {
			return Err(SerializeError {
				ty: SerializeErrorType::ArrayTooLarge(0xFFFF),
				trace: vec![],
			});
		}

		self.serialize_u16(len as u16)?;
		self.serialize_seq(seq)
	}
}

impl Serialize for u8 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_u8(*self)
	}
}
impl Serialize for u16 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_u16(*self)
	}
}
impl Serialize for u32 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_u32(*self)
	}
}

impl Serialize for i8 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_i8(*self)
	}
}
impl Serialize for i16 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_i16(*self)
	}
}
impl Serialize for i32 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_i32(*self)
	}
}

impl Serialize for f32 {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_f32(*self)
	}
}

impl Serialize for bool {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		ser.serialize_u8(if *self { 1 } else { 0 })
	}
}

impl<'a, T> Serialize for &'a T
where
	T: Serialize,
{
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		(*self).serialize(ser)
	}
}

impl<T, U> Serialize for (T, U)
where
	T: Serialize,
	U: Serialize,
{
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		self.0.serialize(ser)?;
		self.1.serialize(ser)
	}
}
