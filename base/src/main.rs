use std::env;

use airmash_server::protocol::GameType;
use airmash_server::resource::RegionName;
use airmash_server::*;
use clap::arg;

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
    .arg(arg!(-c --config [FILE] "Provides an alternate config file"))
    .arg(arg!(--port   [PORT]    "Port that the server will listen on"))
    .arg(arg!(--region [REGION]  "The region that this server belongs to"))
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

  let mut config = airmash_server::config::GamePrototype::default();
  if let Some(path) = matches.value_of("config") {
    let script = match std::fs::read_to_string(path) {
      Ok(script) => script,
      Err(e) => {
        eprintln!("Unable to open config file. Error was {}", e);
        std::process::exit(1);
      }
    };

    config
      .patch(&script)
      .expect("Error while running config file");
  }

  game
    .resources
    .write::<airmash_server::resource::GameConfig>()
    .inner = airmash_server::resource::Config::new(config.clone()).unwrap();
  game
    .resources
    .insert(airmash_server::resource::Config::new(config).unwrap());

  game.run_until_shutdown();
}
