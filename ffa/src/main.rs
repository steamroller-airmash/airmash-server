use std::env;
use std::fs::File;
use std::time::Duration;

use clap::arg;
use serde_deserialize_over::DeserializeOver;
use upgrades::PeriodicPowerupSpawner;

use server::*;
use server::{
  protocol::GameType,
  resource::{Config, RegionName},
};

mod systems;
mod upgrades;

fn set_default_var(name: &str, value: &str) {
  if None == env::var_os(name) {
    env::set_var(name, value);
  }
}

fn main() {
  let matches = clap::Command::new("airmash-server-ffa")
    .version(env!("CARGO_PKG_VERSION"))
    .author("STEAMROLLER")
    .about("Airmash FFA server")
    .arg(arg!(-c --config [FILE] "Provides an alternate config file"))
    .arg(arg!(--port   [PORT]    "Port that the server will listen on"))
    .arg(arg!(--region [REGION]  "The region that this server belongs to"))
    .get_matches();

  set_default_var("RUST_BACKTRACE", "full");
  set_default_var("RUST_LOG", "info");
  env_logger::init();
  color_backtrace::install();

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

  // Use the provided FFA scoreboard systems.
  server::system::ffa::register_all(&mut game);

  // Inferno in Europe
  game.register(PeriodicPowerupSpawner::inferno(
    Vector2::new(920.0, -2800.0),
    Duration::from_secs(105),
  ));
  game.register(PeriodicPowerupSpawner::inferno(
    Vector2::new(-7440.0, -1360.0),
    Duration::from_secs(105),
  ));
  game.register(PeriodicPowerupSpawner::inferno(
    Vector2::new(6565.0, -935.0),
    Duration::from_secs(105),
  ));

  if let Some(path) = matches.value_of("config") {
    let file = match File::open(path) {
      Ok(x) => x,
      Err(e) => {
        eprintln!("Unable to open config file. Error was {}", e);
        return;
      }
    };

    let mut de = serde_json::Deserializer::new(serde_json::de::IoRead::new(file));

    let mut config = Config {
      allow_spectate_while_moving: false,
      ..Config::default()
    };
    config.deserialize_over(&mut de).unwrap();

    game.resources.insert(config);
  }

  game.run_until_shutdown();
}
