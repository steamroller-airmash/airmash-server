use std::time::Duration;

use airmash::protocol::{PlaneType, ServerPacket};
use airmash::test::TestGame;

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

/// This test validates that issue #201 is actually fixed. If we send
/// PLAYER_TYPE packets after PLAYER_RESPAWN packets then we'll find that the
/// plane type used by the client for determining whether Q and E are enabled -
/// and only for that - is one behind what it should be.
#[test]
fn player_type_before_player_respawn() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  client.login("test", &mut game);

  game.run_for(Duration::from_secs(2));

  // Drain all previously sent packets
  let _ = client.packets().count();
  client.send_command("respawn", "3");
  game.run_once();

  // Verify that PLAYER_TYPE occurs before PLAYER_RESPAWN
  assert!(client
    .packets()
    .any(|p| matches!(p, ServerPacket::PlayerType(_))));
  assert!(client
    .packets()
    .any(|p| matches!(p, ServerPacket::PlayerRespawn(_))));
}
