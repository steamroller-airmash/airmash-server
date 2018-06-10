
pub use protocol::ser::Serializer;
pub use protocol::de::Deserializer;
pub use protocol::error::{SerError, DeError};

pub trait Serialize {
    fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError>;
}
pub trait Deserialize<'de> {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<Self, DeError>
    where
        Self: Sized;
}
