use error::{DeserializeError, SerializeError};
use serde::{Deserializer, Serializer};

pub trait Serialize {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError>;
}

pub trait Deserialize: Sized {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError>;
}
