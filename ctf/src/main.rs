use airmash::AirmashGame;
use serde_deserialize_over::DeserializeOver;

fn set_default_var(name: &str, value: &str) {
  use std::env;

  if None == env::var_os(name) {
    env::set_var(name, value);
  }
}

fn main() {
  use airmash::resource::{Config, RegionName};
  use clap::arg;
  use std::env;
  use std::fs::File;

  let matches = clap::Command::new("airmash-server-ctf")
    .version(env!("CARGO_PKG_VERSION"))
    .author("STEAMROLLER")
    .about("Airmash CTF server")
    .arg(arg!(-c --config <FILE> "Provides an alternate config file"))
    .arg(arg!(--port   <PORT>    "Port that the server will listen on"))
    .arg(arg!(--region <REGION>  "The region that this server belongs to"))
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

  airmash_server_ctf::setup_ctf_server(&mut game);

  game.run_until_shutdown();
}
