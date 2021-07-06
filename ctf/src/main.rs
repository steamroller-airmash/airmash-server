#![feature(async_await)]

#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate airmash_server;

use airmash_server as server;

mod component;
mod config;
mod consts;
mod gamemode;
mod shuffle;
mod systems;
mod tasks;

#[cfg(test)]
mod tests;

use std::env;
use std::fs::File;

use serde_deserialize_over::DeserializeOver;

use crate::gamemode::{CTFGameMode, BLUE_TEAM, RED_TEAM};
use crate::server::{AirmashServer, AirmashServerConfig, Config};

#[cfg(features = "sentry")]
/// NOTE: Also initializes env_logger
fn init_sentry() -> Option<sentry::internals::ClientInitGuard> {
  if let Ok(dsn) = env::var("SENTRY_DSN") {
    let guard = sentry::init(&*dsn);

    sentry::integrations::env_logger::init(None, Default::default());
    sentry::integrations::panic::register_panic_handler();

    Some(guard)
  } else {
    env_logger::init();

    None
  }
}

#[cfg(not(features = "sentry"))]
fn init_sentry() {
  env_logger::init();
}

fn main() {
  env::set_var("RUST_BACKTRACE", "1");
  env::set_var("RUST_LOG", "info");

  let matches = clap::App::new("airmash-server-ctf")
    .version(env!("CARGO_PKG_VERSION"))
    .author("STEAMROLLER")
    .about("Airmash CTF server")
    .args_from_usage("-c, --config=[FILE] 'Provides an alternate config file'")
    .get_matches();

  let _guard = init_sentry();

  let mut config = AirmashServerConfig::new("0.0.0.0:3501", CTFGameMode::new()).with_engine();

  config.builder = systems::register(&mut config.world, config.builder);
  config.world.add_resource(shuffle::get_shuffle());

  if let Some(path) = matches.value_of("config") {
    let file = match File::open(path) {
      Ok(x) => x,
      Err(e) => {
        eprintln!("Unable to open config file. Error was {}", e);
        return;
      }
    };

    let mut de = serde_json::Deserializer::new(serde_json::de::IoRead::new(file));

    let mut serverconfig = Config::default();
    serverconfig.deserialize_over(&mut de).unwrap();

    config.world.add_resource(serverconfig);
  }

  AirmashServer::new(config).run().unwrap();
}
