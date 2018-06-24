#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate airmash_server;
extern crate fnv;
extern crate htmlescape;
extern crate shred;
extern crate shrev;
extern crate specs;

use airmash_server as server;

mod component;
mod systems;

use server::AirmashServer;

fn main() {
    let mut server = AirmashServer::new("0.0.0.0:3501").with_engine();

    server.builder = systems::register(&mut server.world, server.builder);

    server.run();
}
