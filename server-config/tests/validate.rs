#![cfg(feature = "script")]

use serde::Deserialize;
use server_config::{GameConfig, GamePrototype};

#[test]
fn default_config_validates() {
  let prototype = GamePrototype::default();
  let lua = rlua::Lua::new();

  lua.context(|lua| {
    let value = prototype
      .patch_direct(lua, "")
      .expect("Failed to run empty patch script");

    let mut track = serde_path_to_error::Track::new();
    let de = serde_rlua::Deserializer::new(value);
    let de = serde_path_to_error::Deserializer::new(de, &mut track);

    let prototype = GamePrototype::deserialize(de)
      .map_err(|e| {
        anyhow::Error::new(e).context(format!("error while deserializing field {}", track.path()))
      })
      .expect("Failed to deserialize config from lua script");

    GameConfig::new(prototype).expect("error while validating the config");
  });
}

#[test]
fn powerup_with_no_server_type_validates() {
  let prototype = GamePrototype::default();
  let lua = rlua::Lua::new();

  lua.context(|lua| {
    let value = prototype
      .patch_direct(
        lua,
        r#"
        data.powerups[1].server_type = nil;
      "#,
      )
      .map_err(anyhow::Error::new)
      .expect("Failed to run empty patch script");

    let mut track = serde_path_to_error::Track::new();
    let de = serde_rlua::Deserializer::new(value);
    let de = serde_path_to_error::Deserializer::new(de, &mut track);

    let prototype = GamePrototype::deserialize(de)
      .map_err(|e| {
        anyhow::Error::new(e).context(format!("error while deserializing field {}", track.path()))
      })
      .expect("Failed to deserialize config from lua script");

    GameConfig::new(prototype).expect("error while validating the config");
  });
}
