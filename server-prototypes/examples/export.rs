#[cfg(not(feature = "script"))]
compile_error!("export example requires the `script` feature to be enabled");

use serde::Deserialize;
use server_prototypes::GamePrototype;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = std::env::args().skip(1).collect::<Vec<_>>();
  // let args = vec!["server-prototypes/configs/infinite-fire.lua"];

  let mut prototype = GamePrototype::default();

  let lua = rlua::Lua::new();
  lua.context(|lua| -> Result<(), Box<dyn std::error::Error>> {
    for file in args {
      let contents = std::fs::read_to_string(file)?;
      let value = prototype.patch_direct(lua, &contents)?;

      let mut track = serde_path_to_error::Track::new();
      let de = serde_rlua::Deserializer::new(value);
      let de = serde_path_to_error::Deserializer::new(de, &mut track);

      match GamePrototype::deserialize(de) {
        Ok(proto) => prototype = proto,
        Err(e) => Err(format!(
          "Error while deserializing field {}: {}",
          track.path(),
          e
        ))?,
      }
    }

    Ok(())
  })?;

  let mut stdout = std::io::stdout().lock();
  serde_json::to_writer_pretty(&mut stdout, &prototype)?;
  stdout.write_all(b"\n")?;

  Ok(())
}
