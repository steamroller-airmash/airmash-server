use server_v2::{
    ecs::{Builder, Entity, World},
    protocol::GameType,
    server::{AirmashServerBuilder, AirmashServerConfig},
    util::GameMode,
    Position, Team,
};

struct EmptyGameMode;

impl GameMode for EmptyGameMode {
    fn assign_team(&mut self, player: Entity) -> Team {
        Team(player.id() as u16 + 3)
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

    let mut world = World::new();
    let mut builder = Builder::new(&mut world);

    builder.with_registrar(server_v2::system::register);
    let dispatch = builder.build().expect("Failed to schedule systems");

    let config = AirmashServerConfig::default();

    AirmashServerBuilder::new(world, config, EmptyGameMode, dispatch)
        .build()
        .run()
        .unwrap();
}
