use std::time::Duration;

use airmash::component::*;
use airmash::event::PlayerKilled;
use airmash::resource::{Config, GameConfig};
use airmash::test::TestGame;
use airmash::util::NalgebraExt;
use airmash::{FireMissileInfo, Vector2};
use airmash_protocol::ServerPacket;

#[test]
fn player_does_not_drop_upgrade_when_not_configured() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  let ent = client.login("test", &mut game);

  game.resources.write::<GameConfig>().spawn_upgrades = false;
  let pred_missile = game
    .resources
    .read::<Config>()
    .missiles
    .get("predator")
    .copied()
    .unwrap();

  game.world.get_mut::<Position>(ent).unwrap().0 = Vector2::zeros();
  game.run_once();

  let missiles = game
    .fire_missiles(
      ent,
      &[FireMissileInfo {
        pos_offset: Vector2::new(0.0, 100.0),
        rot_offset: 0.0,
        proto: pred_missile,
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

#[test]
fn picking_up_an_upgrade_gives_an_upgrade() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  let player = client.login("test", &mut game);

  game.world.get_mut::<Position>(player).unwrap().0 = Vector2::zeros();
  game.run_once();
  game.spawn_mob(MobType::Upgrade, Vector2::zeros(), Duration::from_secs(60));
  game.run_once();

  let num_upgrades = client
    .packets()
    .filter_map(|p| match p {
      ServerPacket::ScoreUpdate(p) => Some(p),
      _ => None,
    })
    .last()
    .expect("Client received no ScoreUpdate packets")
    .upgrades;

  assert_eq!(num_upgrades, 1);
}
