use std::time::Duration;

use airmash::protocol::{MobType, ServerPacket};
use airmash::test::TestGame;

#[test]
fn predator_fires_predator_missile() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  client.login("test", &mut game);

  game.run_for(Duration::from_secs(1));

  client.send_key(airmash_protocol::KeyCode::Fire, true);
  game.run_once();

  loop {
    match client.next_packet() {
      Some(ServerPacket::PlayerFire(evt)) => {
        assert_eq!(evt.projectiles.len(), 1);
        assert_eq!(evt.projectiles[0].ty, MobType::PredatorMissile);
        break;
      }
      Some(_) => (),
      None => panic!("Never received PlayerFire packet"),
    }
  }
}
