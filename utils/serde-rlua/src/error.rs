use std::error::Error as StdError;
use std::fmt;

use rlua::Error as LuaError;
use serde::{de, ser};

/// Error for when a conversion between rust values and [`rlua`] [`Value`]s
/// fails.
///
/// This type is just a wrapper around [`rlua::Error`] that provides some useful
/// trait implementations as needed by the [`Serializer`] and [`Deserializer`]
/// types. Usually you won't want to use this type but instead just convert it
/// directly to [`rlua::Error`]. The [`to_value`] and [`from_value`] helper
/// methods will do this for you.
///
/// [`Value`]: rlua::Value
/// [`Serializer`]: crate::Serializer
/// [`Deserializer`]: crate::Deserializer
/// [`to_value`]: crate::to_value
/// [`from_value`]: crate::from_value
#[derive(Clone, Debug)]
pub struct Error(LuaError);

impl From<LuaError> for Error {
  fn from(e: LuaError) -> Self {
    Self(e)
  }
}

impl From<Error> for LuaError {
  fn from(e: Error) -> Self {
    e.0
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    Some(&self.0)
  }
}

impl ser::Error for Error {
  fn custom<T>(msg: T) -> Self
  where
    T: fmt::Display,
  {
    Self(LuaError::ToLuaConversionError {
      from: "serialize",
      to: "value",
      message: Some(msg.to_string()),
    })
  }
}

impl de::Error for Error {
  fn custom<T>(msg: T) -> Self
  where
    T: fmt::Display,
  {
    Self(LuaError::FromLuaConversionError {
      from: "value",
      to: "deserialize",
      message: Some(msg.to_string()),
    })
  }
}
