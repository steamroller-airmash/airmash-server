use std::time::Duration;

use airmash::component::*;
use airmash::protocol::{server as s, ServerPacket};
use airmash::test::TestGame;
use airmash::util::NalgebraExt;
use airmash::Vector2;

#[test]
fn player_is_upgraded_on_collision_with_upgrade() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  let player = client.login("test", &mut game);
  let powerup = game.spawn_mob(MobType::Inferno, Vector2::zeros(), Duration::from_secs(60));

  game.world.get_mut::<Position>(player).unwrap().0 = Vector2::zeros();
  game.run_once();

  assert!(
    !game.world.contains(powerup),
    "Powerup was not despawned despite having collided with a player"
  );

  let effects = game.world.get::<Effects>(player).unwrap();

  assert!(matches!(effects.powerup(), Some(PowerupType::Inferno)));
}

#[test]
fn dual_powerup_collision() {
  let (mut game, mut mock) = TestGame::new();

  let mut client1 = mock.open();
  let mut client2 = mock.open();

  let p1 = client1.login("p1", &mut game);
  let p2 = client2.login("p2", &mut game);

  game.world.get_mut::<Position>(p1).unwrap().0 = Vector2::zeros();
  game.world.get_mut::<Position>(p2).unwrap().0 = Vector2::zeros();

  game.run_for(Duration::from_secs(4));
  let powerup = game.spawn_mob(MobType::Inferno, Vector2::zeros(), Duration::from_secs(60));
  game.run_once();

  assert!(
    !game.world.contains(powerup),
    "Powerup was not despawned despite having collided with a player"
  );

  let p1pow = game.world.get::<Effects>(p1).unwrap();
  let p2pow = game.world.get::<Effects>(p2).unwrap();

  assert!(p1pow.powerup().is_some() != p2pow.powerup().is_some());
}

#[test]
fn inferno_slows_down_plane() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  let entity = client.login("test", &mut game);

  game.world.get_mut::<Position>(entity).unwrap().0 = Vector2::zeros();
  game.spawn_mob(MobType::Inferno, Vector2::zeros(), Duration::from_secs(60));
  game.run_for(Duration::from_secs(2));

  assert!(client.packets().any(|p| matches!(
    p,
    ServerPacket::PlayerPowerup(s::PlayerPowerup {
      ty: PowerupType::Inferno,
      ..
    })
  )));

  let has_inferno = client
    .packets()
    .filter_map(|p| match p {
      ServerPacket::PlayerUpdate(p) => Some(p),
      _ => None,
    })
    .any(|p| p.upgrades.inferno);
  assert!(has_inferno);
}

#[test]
fn first_spawn_has_2s_shield() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  client.login("test", &mut game);
  game.run_once();

  assert!(client.packets().any(|p| matches!(
    p,
    ServerPacket::PlayerPowerup(s::PlayerPowerup {
      ty: PowerupType::Shield,
      ..
    })
  )));

  game.run_for(Duration::from_secs(3));
  let _ = client.packets().count();
  game.run_for(Duration::from_secs(3));

  let no_shield = !client
    .packets()
    .filter_map(|p| match p {
      ServerPacket::PlayerUpdate(p) => Some(p),
      _ => None,
    })
    .any(|p| p.upgrades.shield);
  assert!(no_shield);
}
