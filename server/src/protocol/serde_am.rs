pub use protocol::de::Deserializer;
pub use protocol::error::{DeError, SerError};
pub use protocol::ser::Serializer;

pub trait Serialize {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError>;
}
pub trait Deserialize<'de> {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self, DeError>
	where
		Self: Sized;
}
