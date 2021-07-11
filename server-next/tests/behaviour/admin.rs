use airmash_server::{component::Position, resource::Config};
use server::protocol::client as c;

#[test]
fn admin_teleport() {
  let (mut game, mut mock) = crate::utils::create_mock_server();

  let mut client = mock.open();
  client.send_login("test");

  game.run_once();

  let id = crate::utils::get_login_id(&mut client);
  let ent = game.get_entity_by_id(id).unwrap();

  game.resources.write::<Config>().admin_enabled = true;

  client.send(c::Command {
    com: "teleport".into(),
    data: "0 -700 2200".into(),
  });

  game.run_once();

  let pos = game.world.get::<Position>(ent).unwrap();

  assert_abs_diff_eq!(pos.x, -700.0, epsilon = 0.1);
  assert_abs_diff_eq!(pos.y, 2200.0, epsilon = 0.1);
}
