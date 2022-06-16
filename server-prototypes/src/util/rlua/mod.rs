//! This module is more or less a port of rlua_serde to work with the most
//! recent rlua and also to fix some small nits around how it produces errors.

mod de;
mod error;
mod ser;

pub(crate) use self::de::LuaDeserializer as Deserializer;
pub(crate) use self::error::Error;
pub(crate) use self::ser::LuaSerializer as Serializer;

#[allow(dead_code)]
pub(crate) fn to_value<'lua, T>(
  lua: rlua::Context<'lua>,
  value: &T,
) -> Result<rlua::Value<'lua>, Error>
where
  T: serde::Serialize,
{
  value.serialize(Serializer::new(lua))
}

#[allow(dead_code)]
pub(crate) fn from_value<'lua, 'de, T>(value: rlua::Value<'lua>) -> Result<T, Error>
where
  T: serde::Deserialize<'de>,
{
  T::deserialize(Deserializer::new(value))
}
