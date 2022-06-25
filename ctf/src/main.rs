use std::time::Duration;

use airmash::util::PeriodicPowerupSpawner;
use airmash::{AirmashGame, Vector2};

fn set_default_var(name: &str, value: &str) {
  use std::env;

  if None == env::var_os(name) {
    env::set_var(name, value);
  }
}

fn main() {
  use std::env;

  use airmash::resource::RegionName;
  use clap::arg;

  let matches = clap::Command::new("airmash-server-ctf")
    .version(env!("CARGO_PKG_VERSION"))
    .author("STEAMROLLER")
    .about("Airmash CTF server")
    .arg(arg!(-c --config [FILE] "Provides an alternate config file"))
    .arg(arg!(--port   [PORT]    "Port that the server will listen on"))
    .arg(arg!(--region [REGION]  "The region that this server belongs to"))
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

  let mut config = airmash::config::GamePrototype::default();
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
    .insert(airmash::resource::Config::new(config).unwrap());

  airmash_server_ctf::setup_ctf_server(&mut game);

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

  // Blue base shield
  game.register(PeriodicPowerupSpawner::shield(
    Vector2::new(-9300.0, -1480.0),
    Duration::from_secs(90),
  ));
  // Red base shield
  game.register(PeriodicPowerupSpawner::shield(
    Vector2::new(8350.0, -935.0),
    Duration::from_secs(90),
  ));

  game.run_until_shutdown();
}
