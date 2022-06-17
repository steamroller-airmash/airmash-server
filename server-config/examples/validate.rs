use std::error::Error;
use std::fmt;

use anyhow::Context;
use serde::Deserialize;
use server_config::{GameConfig, GamePrototype};

#[derive(Clone, Debug)]
struct ValidationError<E> {
  message: String,
  error: E,
}

impl<E> ValidationError<E> {
  pub fn new<D: fmt::Display>(message: D, error: E) -> Self {
    Self {
      message: message.to_string(),
      error,
    }
  }
}

impl<E> fmt::Display for ValidationError<E> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.message)
  }
}

impl<E: Error + 'static> Error for ValidationError<E> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.error)
  }
}

fn main() -> anyhow::Result<()> {
  let mut success = true;
  let args = std::env::args().skip(1).collect::<Vec<_>>();
  // let args = vec!["server-prototypes/configs/infinite-fire.lua"];

  for config in args {
    let prototype = GamePrototype::default();
    let lua = rlua::Lua::new();
    let contents = std::fs::read_to_string(&config)?;

    let result = lua
      .context(|lua| -> anyhow::Result<()> {
        let value = prototype.patch_direct(lua, &contents)?;

        let mut track = serde_path_to_error::Track::new();
        let de = serde_rlua::Deserializer::new(value);
        let de = serde_path_to_error::Deserializer::new(de, &mut track);

        let proto = match GamePrototype::deserialize(de) {
          Ok(proto) => proto,
          Err(e) => {
            return Err(anyhow::Error::new(ValidationError::new(
              format_args!("error whlie deserializing field {}", track.path()),
              e,
            )));
          }
        };

        if let Err(e) = GameConfig::new(proto) {
          return Err(anyhow::Error::new(ValidationError::new(
            "error while validating config",
            e,
          )));
        }

        Ok(())
      })
      .with_context(|| format!("Config `{}` failed to validate", config));

    if let Err(e) = result {
      eprintln!("{:?}", e);
      success = false;
    }
  }

  if !success {
    std::process::exit(1);
  }

  eprintln!("All configs validated successfully!");

  Ok(())
}
