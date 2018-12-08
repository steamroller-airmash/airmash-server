#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate airmash_server;
extern crate env_logger;
extern crate fnv;
extern crate htmlescape;
extern crate rand;
extern crate shred;
extern crate shrev;
extern crate specs;
extern crate sentry;
extern crate clap;

use airmash_server as server;

mod component;
mod config;
mod consts;
mod gamemode;
mod shuffle;
mod systems;

use std::env;
use std::fs::File;

use gamemode::{CTFGameMode, BLUE_TEAM, RED_TEAM};
use server::{AirmashServer, AirmashServerConfig, Config};

/// NOTE: Also initializes env_logger
fn init_sentry() -> Option<sentry::internals::ClientInitGuard>{
	if let Ok(dsn) = env::var("SENTRY_DSN") {
		let guard = sentry::init(&*dsn);

		sentry::integrations::env_logger::init(None, Default::default());
		sentry::integrations::panic::register_panic_handler();

		Some(guard)
	}
	else {
		env_logger::init();

		None
	}
}

fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	env::set_var("RUST_LOG", "airmash_server=info");
	
	let matches = clap::App::new("airmash-server-ctf")
			.version(env!("CARGO_PKG_VERSION"))
			.author("STEAMROLLER")
			.about("Airmash CTF server")
			.args_from_usage("-c, --config=[FILE] 'Provides an alternate config file'")
			.get_matches();

	let _guard = init_sentry();

	let mut config = AirmashServerConfig::new("0.0.0.0:3501", CTFGameMode::new())
		.with_engine()
		.with_alpha_warning();

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

			let serverconfig: Config = serde_json::from_reader(file).unwrap_or_else(|e| {
					error!("Unable to parse config file! Using default config.");
					error!("Config file error was: {}", e);
					Default::default()
			});

			config.world.add_resource(serverconfig);
	}

	AirmashServer::new(config).run().unwrap();
}
