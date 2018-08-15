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
	env_logger::init();

	env::set_var("RUST_BACKTRACE", "1");

	AirmashServer::new("0.0.0.0:3501")
		.with_engine()
		.with_gamemode(EmptyGameMode)
		.run();
}
