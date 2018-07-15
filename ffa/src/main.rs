extern crate airmash_server;
extern crate log;
extern crate simple_logger;
extern crate specs;
extern crate shred;
extern crate rand;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;

mod systems;
mod gamemode;
mod components;

use std::env;

use gamemode::EmptyGameMode;

use airmash_server::*;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    env::set_var("RUST_BACKTRACE", "1");

    let mut server = AirmashServer::new("0.0.0.0:3501")
        .with_engine()
        .with_gamemode(EmptyGameMode);

    server.builder = systems::register(server.builder);

    server.run();
}
