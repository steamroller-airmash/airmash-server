
pub use protocol::ser::Serializer;
pub use protocol::de::Deserializer;
pub use protocol::error::Result;

pub trait Serialize {
    fn serialize(&self, ser: &mut Serializer) -> Result<()>;
}
pub trait Deserialize<'de> {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<Self>
    where
        Self: Sized;
}
