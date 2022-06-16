use rlua::Error as LuaError;
use serde::{de, ser};
use std::error::Error as StdError;
use std::fmt;

#[derive(Clone, Debug)]
pub(crate) struct Error(LuaError);

impl From<LuaError> for Error {
  fn from(e: LuaError) -> Self {
    Self(e)
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
