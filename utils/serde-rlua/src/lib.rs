//! This module is more or less a port of rlua_serde to work with the most
//! recent rlua and also to fix some small nits around how it produces errors.

#[cfg(test)]
#[macro_use]
extern crate serde;

mod de;
mod error;
mod ser;

pub use self::de::LuaDeserializer as Deserializer;
pub use self::error::Error;
pub use self::ser::LuaSerializer as Serializer;

pub fn to_value<'lua, T>(
  lua: rlua::Context<'lua>,
  value: &T,
) -> Result<rlua::Value<'lua>, rlua::Error>
where
  T: serde::Serialize,
{
  value.serialize(Serializer::new(lua)).map_err(From::from)
}

pub fn from_value<'lua, 'de, T>(value: rlua::Value<'lua>) -> Result<T, rlua::Error>
where
  T: serde::Deserialize<'de>,
{
  T::deserialize(Deserializer::new(value)).map_err(From::from)
}
