use error::*;
use serde::*;

pub fn serialize<T: Serialize>(s: &[T], ser: &mut Serializer) -> Result<(), SerializeError> {
	ser.serialize_array_large(s)
}
pub fn deserialize<'de, T: Deserialize>(
	de: &mut Deserializer<'de>,
) -> Result<Vec<T>, DeserializeError> {
	de.deserialize_array_large()
}
