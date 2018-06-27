
extern crate log;
extern crate specs;
extern crate simple_logger;
extern crate airmash_server;

use specs::Entity;
use airmash_server::*;
use airmash_server::protocol::GameType;

struct EmptyGameMode;

impl GameMode for EmptyGameMode {
    fn assign_team(&mut self, player: Entity) -> Team {
        Team(player.id() as u16)
    }
    fn respawn_pos(&mut self, _: Entity, _: Team) -> Position {
        Position::default()
    }
    fn gametype(&self) -> GameType {
        GameType::FFA
    }
}

fn main() {
    AirmashServer::new("0.0.0.0:3501")
        .with_engine()
        .with_gamemode(EmptyGameMode)
        .run();
}
