
use protocol::serde_am::*;
use protocol::error::Error;

/// All plane types present within airmash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum PlaneType {
    Predator,
    Tornado,
    Mohawk,
    Goliath,
    Prowler
}

const PREDATOR: u8 = 1;
const GOLIATH: u8 = 2;
const MOHAWK: u8 = 3;
const TORNADO: u8 = 4;
const PROWLER: u8 = 5;

impl PlaneType {
    /// Get the associated numerical value
    pub fn to_u8(self) -> u8 {
        match self {
            PlaneType::Predator => PREDATOR,
            PlaneType::Tornado =>  TORNADO,
            PlaneType::Mohawk =>   MOHAWK,
            PlaneType::Goliath =>  GOLIATH,
            PlaneType::Prowler =>  PROWLER
        }
    }

    /// Get the plane associated with a numerical
    /// value, or `None` otherwise.
    pub fn from_u8(v: u8) -> Option<Self> {
        Some(match v {
            PREDATOR => PlaneType::Predator,
            TORNADO  => PlaneType::Tornado,
            MOHAWK   => PlaneType::Mohawk,
            GOLIATH  => PlaneType::Goliath,
            PROWLER  => PlaneType::Prowler,
            _ => return None
        })
    }
}

impl Serialize for PlaneType {
    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        self.to_u8().serialize(ser)
    }
}
impl<'de> Deserialize<'de> for PlaneType {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
        let ival = de.deserialize_u8()?;
        match Self::from_u8(ival) {
            Some(v) => Ok(v),
            None => Err(Error::InvalidPlaneType(ival))
        }
    }
}
