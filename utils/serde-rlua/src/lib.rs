//! Serde serializers and deserializers for deserializing to and from [`rlua`]
//! [`Value`]s.
//!
//! It is more or less a port of [`rlua_serde`] that has been made to work with
//! the most recent version of [`rlua`] and made to provide better error
//! messages when things fail.
//!
//! [`Value`]: rlua::Value
//! [`rlua_serde`]: https://crates.io/crates/rlua_serde

#[cfg(test)]
#[macro_use]
extern crate serde;

mod de;
mod error;
mod ser;

pub use self::de::LuaDeserializer as Deserializer;
pub use self::error::Error;
pub use self::ser::LuaSerializer as Serializer;

/// Convert a rust value to a [`Value`].
///
/// This is a simple wrapper around `serialize` and [`Serializer`] that returns
/// a [`rlua::Error`] so that it can be more easily used in [`rlua::ToLua`]
/// implementations.
///
/// [`Value`]: rlua::Value
pub fn to_value<'lua, T>(
  lua: rlua::Context<'lua>,
  value: &T,
) -> Result<rlua::Value<'lua>, rlua::Error>
where
  T: serde::Serialize,
{
  value.serialize(Serializer::new(lua)).map_err(From::from)
}

/// Convert a [`Value`] to a rust value.
///
/// This is a simple wrapper around `serialize` and [`Serializer`] that returns
/// a [`rlua::Error`] so that it can be more easily used in [`rlua::FromLua`]
/// implementations.
///
/// [`Value`]: rlua::Value
pub fn from_value<'lua, 'de, T>(value: rlua::Value<'lua>) -> Result<T, rlua::Error>
where
  T: serde::Deserialize<'de>,
{
  T::deserialize(Deserializer::new(value)).map_err(From::from)
}
