#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate fnv;
extern crate rand;
extern crate shred;
extern crate shrev;
extern crate specs;
extern crate htmlescape;
extern crate airmash_server;

use airmash_server as server;

mod component;
mod systems;
mod gamemode;

use server::AirmashServer;
use gamemode::CTFGameMode;

fn main() {
    let mut server = AirmashServer::new("0.0.0.0:3501")
			.with_engine()
			.with_gamemode(CTFGameMode::new());

    server.builder = systems::register(&mut server.world, server.builder);

    server.run();
}
