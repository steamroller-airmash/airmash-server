use std::mem;
use std::str;

use protocol::error::DeError;
use protocol::serde_am::Deserialize;

pub struct Deserializer<'a> {
    pub bytes: &'a [u8],
}

/// Deserializes a struct from a byte buffer,
/// and returns an [`Error`](enum.error.html)
/// when the bytes cannot be deserialized 
/// to the given type. 
/// 
/// # Example
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::{from_bytes, ClientPacket};
/// # fn main() {
/// // Bytes representing an Ack packet
/// let bytes = [ 5 ];
/// 
/// // Decode the packet
/// let ack = from_bytes::<ClientPacket>(&bytes).unwrap();
/// 
/// // Do stuff with packet here...
/// match ack {
///     Ack => return,
///     _ => panic!("This wasn't an ack packet!")
/// }
/// # }
/// ```
/// 
/// # Error Example
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::{from_bytes, ServerPacket, Error};
/// # use airmash_protocol::server::ChatPublic;
/// # fn main() { // Need for extern crate to work
/// // An incomplete ChatPublic packet.
/// // In this case, the packet is too short.
/// let bytes = [ 70 ];
/// 
/// // Try to decode the packet
/// let result = from_bytes::<ChatPublic>(&bytes);
/// 
/// // Do something with the error here...
/// match result {
///     Ok(_) => panic!("Unexpected success!"),
///     Err(err) => return
/// }
/// # }
/// ```
pub fn from_bytes<'a, T>(b: &'a [u8]) -> Result<T, DeError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(b);
    let t = T::deserialize(&mut deserializer)?;

    if deserializer.bytes.is_empty() {
        Ok(t)
    } else {
        Err(DeError::TrailingBytes)
    }
}

impl<'a> Deserializer<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        return Self { bytes };
    }
}

impl<'de> Deserializer<'de> {
    pub fn deserialize_i8(&mut self) -> Result<i8, DeError> {
        Ok(self.deserialize_u8()? as i8)
    }
    pub fn deserialize_i16(&mut self) -> Result<i16, DeError> {
        Ok(self.deserialize_u16()? as i16)
    }
    pub fn deserialize_i32(&mut self) -> Result<i32, DeError> {
        Ok(self.deserialize_u32()? as i32)
    }
    pub fn deserialize_i64(&mut self) -> Result<i64, DeError> {
        Ok(self.deserialize_u64()? as i64)
    }

    pub fn deserialize_u8(&mut self) -> Result<u8, DeError> {
        if self.bytes.len() == 0 {
            return Err(DeError::Eof);
        }

        let b = self.bytes[0];
        self.bytes = &self.bytes[1..];
        Ok(b)
    }
    pub fn deserialize_u16(&mut self) -> Result<u16, DeError> {
        let lo = self.deserialize_u8()?;
        let hi = self.deserialize_u8()?;

        Ok(((hi as u16) << 8) | (lo as u16))
    }
    pub fn deserialize_u32(&mut self) -> Result<u32, DeError> {
        let lo = self.deserialize_u16()?;
        let hi = self.deserialize_u16()?;

        Ok(((hi as u32) << 16) | (lo as u32))
    }
    pub fn deserialize_u64(&mut self) -> Result<u64, DeError> {
        let lo = self.deserialize_u32()?;
        let hi = self.deserialize_u32()?;

        Ok(((hi as u64) << 32) | (lo as u64))
    }

    pub fn deserialize_f32(&mut self) -> Result<f32, DeError> {
        Ok(unsafe { mem::transmute::<u32, f32>(self.deserialize_u32()?) })
    }
    pub fn deserialize_f64(&mut self) -> Result<f64, DeError> {
        Ok(unsafe { mem::transmute::<u64, f64>(self.deserialize_u64()?) })
    }

    pub fn deserialize_unit(&mut self) -> Result<(), DeError> {
        Ok(())
    }
    pub fn deserialize_bytes(&mut self, len: usize) -> Result<&'de [u8], DeError> {
        if self.bytes.len() < len {
            return Err(DeError::Eof);
        }

        let slice = &self.bytes[0..len];
        self.bytes = &self.bytes[len..];
        Ok(slice)
    }
    pub fn deserialize_str(&mut self, len: usize) -> Result<&'de str, DeError> {
        let bytes = self.deserialize_bytes(len)?;

        match str::from_utf8(bytes) {
            Ok(val) => Ok(val),
            Err(e) => Err(DeError::Utf8Error(e)),
        }
    }
}
