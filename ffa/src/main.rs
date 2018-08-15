extern crate airmash_server;
extern crate env_logger;
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

mod components;
mod gamemode;
mod systems;

use std::env;

use gamemode::EmptyGameMode;

use airmash_server::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut server = AirmashServer::new("0.0.0.0:3501")
        .with_engine()
        .with_gamemode(EmptyGameMode);

    server.builder = systems::register(server.builder);

    server.run();
}
