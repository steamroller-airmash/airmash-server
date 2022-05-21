use std::time::Duration;

use server::component::*;
use server::test::TestGame;
use server::Vector2;

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

  let powerup = game.world.get::<Powerup>(player).unwrap();

  assert!(powerup.data.is_some());

  if let Some(data) = &powerup.data {
    assert_eq!(data.ty, PowerupType::Inferno);
  }
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

  let p1pow = game.world.get::<Powerup>(p1).unwrap();
  let p2pow = game.world.get::<Powerup>(p2).unwrap();

  assert!(p1pow.data.is_some() != p2pow.data.is_some());
}
