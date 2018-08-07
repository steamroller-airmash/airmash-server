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
extern crate rand;
extern crate shred;
extern crate shrev;
extern crate simple_logger;
extern crate specs;

use airmash_server as server;

mod component;
mod config;
mod consts;
mod gamemode;
mod shuffle;
mod systems;

use std::env;

use gamemode::{CTFGameMode, BLUE_TEAM, RED_TEAM};
use server::AirmashServer;

fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	simple_logger::init_with_level(log::Level::Info).unwrap();

	let mut server = AirmashServer::new("0.0.0.0:3501")
		.with_engine()
		.with_gamemode(CTFGameMode::new())
		.with_alpha_warning();

	server.builder = systems::register(&mut server.world, server.builder);
	server.world.add_resource(shuffle::get_shuffle());

	server.run();
}
