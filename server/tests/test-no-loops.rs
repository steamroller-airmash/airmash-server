extern crate airmash_server;
extern crate specs;

use airmash_server::protocol::GameType;
use airmash_server::{AirmashServerConfig, GameMode, Position, Team};
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

#[test]
fn no_system_dependency_loops() {
	let config = AirmashServerConfig::new("0.0.0.0:3501", EmptyGameMode).with_engine();

	config.builder.build();
}
