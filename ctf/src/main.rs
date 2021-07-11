//! Airmash CTF server.

#[macro_use]
extern crate log;
#[macro_use]
extern crate airmash;

// mod component;
// mod config;
// mod consts;
// mod gamemode;
// mod shuffle;
// mod systems;
// mod tasks;

mod component;
mod config;
mod event;
mod resource;
mod shuffle;
mod systems;

use std::time::Instant;

use airmash::AirmashGame;
use serde_deserialize_over::DeserializeOver;

fn set_default_var(name: &str, value: &str) {
  use std::env;

  if None == env::var_os(name) {
    env::set_var(name, value);
  }
}

fn setup_flag_entities(game: &mut AirmashGame) {
  use crate::component::*;
  use crate::config::{BLUE_TEAM, RED_TEAM};

  use airmash::component::*;

  game.world.spawn((
    Position(config::flag_home_pos(RED_TEAM)),
    Team(RED_TEAM),
    FlagCarrier(None),
    LastDrop {
      player: None,
      time: Instant::now(),
    },
    LastReturnTime(Instant::now()),
    IsFlag,
  ));

  game.world.spawn((
    Position(config::flag_home_pos(BLUE_TEAM)),
    Team(BLUE_TEAM),
    FlagCarrier(None),
    LastDrop {
      player: None,
      time: Instant::now(),
    },
    LastReturnTime(Instant::now()),
    IsFlag,
  ));

  info!(" red flag: {:?}", config::flag_home_pos(RED_TEAM));
  info!("blue flag: {:?}", config::flag_home_pos(BLUE_TEAM));
}

fn main() {
  use airmash::resource::{Config, GameType, RegionName};
  use std::env;
  use std::fs::File;

  let matches = clap::App::new("airmash-server-ctf")
    .version(env!("CARGO_PKG_VERSION"))
    .author("STEAMROLLER")
    .about("Airmash CTF server")
    .arg_from_usage("-c, --config=[FILE] 'Provides an alternate config file'")
    .arg_from_usage("--port=[PORT]       'Port that the server will listen on'")
    .arg_from_usage("--region=[REGION]   'The region that this server belongs to'")
    .get_matches();

  set_default_var("RUST_BACKTRACE", "1");
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
  game.resources.insert(GameType::CTF);

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

  setup_flag_entities(&mut game);
  resource::register_all(&mut game);
  crate::airmash::system::ctf::register_all(&mut game);

  game.run_until_shutdown();
}
