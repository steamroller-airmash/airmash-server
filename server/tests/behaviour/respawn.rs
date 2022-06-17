use std::time::Duration;

use server::protocol::{PlaneType, ServerPacket};
use server::test::TestGame;

#[test]
fn respawn_as_mohawk() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  client.login("test", &mut game);

  game.run_for(Duration::from_secs(2));
  client.send_command("respawn", "3");
  game.run_once();

  loop {
    match client.next_packet() {
      Some(ServerPacket::PlayerType(evt)) => {
        assert_eq!(evt.ty, PlaneType::Mohawk);
        break;
      }
      Some(_) => (),
      None => panic!("Never received PlayerType packet"),
    }
  }
}
