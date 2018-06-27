
extern crate specs;
extern crate airmash_server;

use specs::Entity;
use airmash_server::*;

struct EmptyGameMode;

impl GameMode for EmptyGameMode {
    fn assign_team(&mut self, player: Entity) -> Team {
        Team(player.id() as u16)
    }
    fn respawn_pos(&mut self, _: Entity, _: Team) -> Position {
        Position::default()
    }
}

fn main() {
    AirmashServer::new("0.0.0.0:3501")
        .with_engine()
        .with_gamemode(EmptyGameMode)
        .run();
}
