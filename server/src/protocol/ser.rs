use protocol::serde_am::{SerError, Serialize};

use std::mem;
use std::result;
use std::vec::Vec;

type Result<T> = result::Result<T, SerError>;

pub struct Serializer {
	pub output: Vec<u8>,
}

/// Serializes a struct to a byte vector, returning
/// an [`Error`](enum.error.html) when the struct
/// cannot be serialized.
///
/// # Example
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::{to_bytes, ClientPacket};
/// # use airmash_protocol::client::TeamChat;
/// # fn main() {
/// // Create a packet to be sent to the server
/// let packet = ClientPacket::TeamChat(TeamChat {
///     text: "The enemy has our flag!".to_string()
/// });
///
/// // Serialize the packet
/// let bytes = to_bytes(&packet).unwrap();
///
/// // Send packet to server here...
/// # }
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
	T: Serialize,
{
	let mut serializer = Serializer { output: vec![] };
	value.serialize(&mut serializer)?;
	Ok(serializer.output)
}

impl Serializer {
	pub fn serialize_i8(&mut self, v: i8) -> Result<()> {
		self.serialize_u8(v as u8)
	}
	pub fn serialize_i16(&mut self, v: i16) -> Result<()> {
		self.serialize_u16(v as u16)
	}
	pub fn serialize_i32(&mut self, v: i32) -> Result<()> {
		self.serialize_u32(v as u32)
	}
	pub fn serialize_i64(&mut self, v: i64) -> Result<()> {
		self.serialize_u64(v as u64)
	}

	pub fn serialize_u8(&mut self, v: u8) -> Result<()> {
		self.output.push(v);
		Ok(())
	}
	pub fn serialize_u16(&mut self, v: u16) -> Result<()> {
		self.serialize_u8(v as u8)?;
		self.serialize_u8((v >> 8) as u8)
	}
	pub fn serialize_u32(&mut self, v: u32) -> Result<()> {
		self.serialize_u16(v as u16)?;
		self.serialize_u16((v >> 16) as u16)
	}
	pub fn serialize_u64(&mut self, v: u64) -> Result<()> {
		self.serialize_u32(v as u32)?;
		self.serialize_u32((v >> 32) as u32)
	}

	pub fn serialize_f32(&mut self, v: f32) -> Result<()> {
		self.serialize_u32(unsafe { mem::transmute::<f32, u32>(v) })
	}
	pub fn serialize_f64(&mut self, v: f64) -> Result<()> {
		self.serialize_u64(unsafe { mem::transmute::<f64, u64>(v) })
	}

	#[allow(dead_code)]
	pub fn serialize_unit(&mut self) -> Result<()> {
		Ok(())
	}
	// Must have a size called separately beforehand
	pub fn serialize_bytes(&mut self, v: &[u8]) -> Result<()> {
		self.output.extend_from_slice(v);
		Ok(())
	}
	#[allow(dead_code)]
	pub fn serialize_bool(&mut self, v: bool) -> Result<()> {
		self.serialize_u8(if v { 1 } else { 0 })
	}
}
