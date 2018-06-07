
use protocol::serde_am::*;
use protocol::error::Error;

/// All the keys that are sent from the client
/// to the server.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,
    Fire,
    Special
}

impl KeyCode {
    /// Get the keycode associated with the
    /// numerical value. Returns `None` otherwise.
    pub fn from_u8(code: u8) -> Option<KeyCode> {
        match code {
            1 => Some(KeyCode::Up),
            2 => Some(KeyCode::Down),
            3 => Some(KeyCode::Left),
            4 => Some(KeyCode::Right),
            5 => Some(KeyCode::Fire),
            6 => Some(KeyCode::Special),
            _ => None
        }
    }

    /// Get the numerical value associated with
    /// the keycode. 
    /// 
    /// This value is the value that is actually
    /// sent over the network.
    pub fn to_u8(self) -> u8 {
        match self {
            KeyCode::Up => 1,
            KeyCode::Down => 2,
            KeyCode::Left => 3,
            KeyCode::Right => 4,
            KeyCode::Fire => 5,
            KeyCode::Special => 6
        }
    }
}

impl Serialize for KeyCode {
    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        ser.serialize_u8(self.to_u8())
    }
}

impl<'de> Deserialize<'de> for KeyCode {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<KeyCode> {
        let ival = de.deserialize_u8()?;
        match KeyCode::from_u8(ival) {
            Some(code) => Ok(code),
            None => Err(Error::InvalidKeyCode(ival))
        }
    }
}



