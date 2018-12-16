extern crate airmash_server;
extern crate env_logger;
extern crate specs;
extern crate serde_json;
extern crate clap;
#[macro_use]
extern crate log;

use std::env;
use std::fs::File;

use airmash_server::protocol::GameType;
use airmash_server::*;
use specs::Entity;

struct EmptyGameMode;

impl GameMode for EmptyGameMode {
	fn assign_team(&mut self, player: Entity) -> Team {
		Team(player.id() as u16)
	}
	fn spawn_pos(&mut self, _: Entity, _: Team) -> Position {
		Position::default()
	}
	fn gametype(&self) -> GameType {
		GameType::FFA
	}
	fn room(&self) -> String {
		"matrix".to_owned()
	}
}

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

	let mut config = AirmashServerConfig::new("0.0.0.0:3501", EmptyGameMode).with_engine();
	
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
