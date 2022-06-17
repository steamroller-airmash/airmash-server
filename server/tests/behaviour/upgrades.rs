use airmash_protocol::ServerPacket;
use airmash_server::component::*;
use airmash_server::event::PlayerKilled;
use airmash_server::resource::GameConfig;
use airmash_server::{FireMissileInfo, Vector2};
use server::test::TestGame;

#[test]
fn player_does_not_drop_upgrade_when_not_configured() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  let ent = client.login("test", &mut game);

  {
    let mut game_config = game.resources.write::<GameConfig>();
    game_config.spawn_upgrades = false;
  }

  game.world.get_mut::<Position>(ent).unwrap().0 = Vector2::zeros();
  game.run_once();

  let missiles = game
    .fire_missiles(
      ent,
      &[FireMissileInfo {
        pos_offset: Vector2::new(0.0, 100.0),
        rot_offset: 0.0,
        ty: MobType::PredatorMissile,
      }],
    )
    .unwrap();
  game.run_once();

  game.dispatch(PlayerKilled {
    player: ent,
    missile: missiles[0],
    killer: None,
  });

  game.run_once();

  while let Some(packet) = client.next_packet() {
    if let ServerPacket::MobUpdateStationary(_) = packet {
      panic!("Upgrade was spawned despite upgrades being disabled");
    }
  }
}
