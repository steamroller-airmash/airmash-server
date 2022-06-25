use airmash::component::*;
use airmash::protocol::KeyCode;
use airmash::resource::Config;
use airmash::Vector2;

#[test]
fn prowler_decloak_on_hit() {
  let (mut game, mut mock) = crate::utils::create_mock_server();

  let &prowler = game
    .resources
    .read::<Config>()
    .planes
    .get("prowler")
    .unwrap();

  let mut client1 = mock.open();
  let mut client2 = mock.open();

  client1.send(crate::utils::create_login_packet("test-1"));
  client2.send(crate::utils::create_login_packet("test-2"));

  game.run_once();

  let id1 = crate::utils::get_login_id(&mut client1);
  let id2 = crate::utils::get_login_id(&mut client2);

  let ent1 = game.find_entity_by_id(id1).unwrap();
  let ent2 = game.find_entity_by_id(id2).unwrap();

  game
    .world
    .insert(ent1, (Position(Vector2::new(0.0, -250.0)), prowler))
    .unwrap();
  game
    .world
    .insert_one(ent2, Position(Vector2::new(0.0, 0.0)))
    .unwrap();

  client1.send_key(KeyCode::Special, true);
  game.run_count(5);
  client1.send_key(KeyCode::Special, false);
  client2.send_key(KeyCode::Fire, true);
  game.run_count(5);
  client2.send_key(KeyCode::Fire, false);

  game.run_count(60);

  let active = game.world.get::<SpecialActive>(ent2).unwrap();
  assert!(!active.0);
}
