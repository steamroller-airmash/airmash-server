extern crate airmash_server;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;
extern crate shred;
extern crate specs;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
extern crate clap;
extern crate serde_json;

mod components;
mod gamemode;
mod systems;

use std::env;
use std::fs::File;

use gamemode::EmptyGameMode;

use airmash_server::types::Config;
use airmash_server::*;

fn main() {
    let matches = clap::App::new("airmash-server-ffa")
        .version(env!("CARGO_PKG_VERSION"))
        .author("STEAMROLLER")
        .about("Airmash FFA server")
        .args_from_usage("-c, --config=[FILE] 'Provides an alternate config file'")
        .get_matches();

    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut server = AirmashServer::new("0.0.0.0:3501")
        .with_engine()
        .with_gamemode(EmptyGameMode);

    server.builder = systems::register(server.builder);

    if let Some(path) = matches.value_of("config") {
        let file = match File::open(path) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Unable to open config file. Error was {}", e);
                return;
            }
        };

        let config: Config = serde_json::from_reader(file).unwrap_or_else(|e| {
            error!("Unable to parse config file! Using default config.");
            error!("Config file error was: {}", e);
            Default::default()
        });

        server.world.add_resource(config);
    }

    server.run();
}
