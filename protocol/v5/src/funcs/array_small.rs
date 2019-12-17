use error::*;
use serde::*;

pub fn serialize<T>(s: &[T], ser: &mut Serializer) -> Result<(), SerializeError>
where
	T: Serialize,
{
	ser.serialize_array_small(s)
}
pub fn deserialize<'de, T>(de: &mut Deserializer<'de>) -> Result<Vec<T>, DeserializeError>
where
	T: Deserialize,
{
	de.deserialize_array_small()
}
