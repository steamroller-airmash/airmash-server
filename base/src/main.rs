extern crate airmash_server;
extern crate env_logger;
extern crate log;
extern crate specs;

use std::env;

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
	env::set_var("RUST_BACKTRACE", "1");
	env::set_var("RUST_LOG", "info");

	env_logger::init();

	let config = AirmashServerConfig::new("0.0.0.0:3501", EmptyGameMode).with_engine();

	AirmashServer::new(config).run().unwrap();
}
