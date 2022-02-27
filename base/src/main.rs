#[macro_use]
extern crate log;

use clap::arg;
use serde_deserialize_over::DeserializeOver;
use std::env;
use std::fs::File;

use airmash_server::protocol::GameType;
use airmash_server::resource::Config;
use airmash_server::resource::RegionName;
use airmash_server::*;

fn set_default_var(name: &str, value: &str) {
  if None == env::var_os(name) {
    env::set_var(name, value);
  }
}

fn main() {
  let matches = clap::Command::new("airmash-server-base")
    .version(env!("CARGO_PKG_VERSION"))
    .author("STEAMROLLER")
    .about("Airmash Test Server")
    .arg(arg!(-c --config <FILE> "Provides an alternate config file"))
    .arg(arg!(--port   <PORT>    "Port that the server will listen on"))
    .arg(arg!(--region <REGION>  "The region that this server belongs to"))
    .get_matches();

  set_default_var("RUST_BACKTRACE", "full");
  set_default_var("RUST_LOG", "info");

  env_logger::init();

  let bind_addr = format!("0.0.0.0:{}", matches.value_of("port").unwrap_or("3501"));

  let mut game = AirmashGame::with_network(
    bind_addr
      .parse()
      .expect("Unable to parse provided network port address"),
  );
  game.resources.insert(RegionName(
    matches.value_of("region").unwrap_or("default").to_string(),
  ));
  game.resources.insert(GameType::FFA);

  // Use the FFA scoreboard.
  airmash_server::system::ffa::register_all(&mut game);

  if let Some(path) = matches.value_of("config") {
    let file = match File::open(path) {
      Ok(x) => x,
      Err(e) => {
        eprintln!("Unable to open config file. Error was {}", e);
        return;
      }
    };

    let mut config = game.resources.write::<Config>();
    let mut de = serde_json::Deserializer::new(serde_json::de::IoRead::new(file));
    if let Err(e) = config.deserialize_over(&mut de) {
      error!("Unable to parse config file: {}", e);
      return;
    }
  }

  game.run_until_shutdown();
}
