#[macro_use]
extern crate log;

use std::env;
use std::fs::File;

use airmash_server::protocol::GameType;
use airmash_server::resource::Config;
use airmash_server::resource::GameRoom;
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

	let mut game = AirmashWorld::with_network("0.0.0.0:3501".parse().unwrap());
	game.resources.insert(GameRoom("matrix".to_owned()));
	game.resources.insert(GameType::FFA);

	if let Some(path) = matches.value_of("config") {
		let file = match File::open(path) {
			Ok(x) => x,
			Err(e) => {
				eprintln!("Unable to open config file. Error was {}", e);
				return;
			}
		};

		let mut config = game.resources.write::<Config>();
		*config = serde_json::from_reader(file).unwrap_or_else(|e| {
			error!("Unable to parse config file! Using default config.");
			error!("Config file error was: {}", e);
			Default::default()
		});
	}

	game.run_until_shutdown();
}
