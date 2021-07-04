#[macro_use]
extern crate log;

use serde_deserialize_over::DeserializeOver;
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
	env::set_var("RUST_LOG", "debug");

	env_logger::init();

	let mut game = AirmashGame::with_network("0.0.0.0:3501".parse().unwrap());
	game.resources.insert(GameRoom("matrix".to_owned()));
	game.resources.insert(GameType::FFA);

	// Use the FFA scoreboard.
	airmash_server::system::ffa::register_all(&mut game);

	if let Some(path) = matches.value_of("config") {
		let file = match File::open(path) {
			Ok(x) => x,
			Err(e) => {
				eprintln!("Unable to open config file. Error was {}", e);
				return;
			}
		};

		let mut config = game.resources.write::<Config>();
		let mut de = serde_json::Deserializer::new(serde_json::de::IoRead::new(file));
		if let Err(e) = config.deserialize_over(&mut de) {
			error!("Unable to parse config file: {}", e);
			return;
		}
	}

	game.run_until_shutdown();
}
