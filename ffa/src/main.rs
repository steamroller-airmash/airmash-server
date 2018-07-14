extern crate airmash_server;
extern crate log;
extern crate simple_logger;
extern crate specs;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use std::env;

use airmash_server::protocol::GameType;
use airmash_server::*;
use specs::Entity;

use rand::Closed01;

lazy_static! {
    static ref SPAWN_TOP_RIGHT: Position = Position::new(
        Distance::new(-1325.0),
        Distance::new(-4330.0)
    );

    static ref SPAWN_SIZE: Position = Position::new(
        Distance::new(3500.0),
        Distance::new(3500.0),
    );
}

struct EmptyGameMode;

impl GameMode for EmptyGameMode {
    fn assign_team(&mut self, player: Entity) -> Team {
        Team(player.id() as u16)
    }
    fn spawn_pos(&mut self, _: Entity, _: Team) -> Position {
        let Closed01(x) = rand::random();
        let Closed01(y) = rand::random();
        let mult: Vector2<f32> = Vector2::new(x, y);

        *SPAWN_TOP_RIGHT + *SPAWN_SIZE * mult
    }
    fn gametype(&self) -> GameType {
        GameType::FFA
    }
    fn room(&self) -> String {
        "matrix".to_owned()
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    env::set_var("RUST_BACKTRACE", "1");

    AirmashServer::new("0.0.0.0:3501")
        .with_engine()
        .with_gamemode(EmptyGameMode)
        .run();
}
