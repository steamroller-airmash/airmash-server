use std::time::Duration;

use server::Vector2;
use server::component::*;
use server::test::TestGame;

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
