use std::time::Duration;

use airmash_protocol::ServerPacket;
use airmash_server::protocol::server::ServerMessage;
use airmash_server::resource::TaskScheduler;
use airmash_server::test::*;

#[test]
fn tasks_obey_test_time() {
  let (mut game, mut mock) = TestGame::new();

  let mut conn = mock.open();
  conn.login("test", &mut game);

  unsafe {
    let sched = game.resources.read::<TaskScheduler>().clone();
    sched.spawn(move |mut game| async move {
      game.sleep_for(Duration::from_secs(5)).await;

      game.send_to_all(ServerMessage {
        ty: airmash_protocol::ServerMessageType::Banner,
        text: "test-message".into(),
        duration: 1000,
      });
    });
  }

  game.run_for(Duration::from_secs(7));

  let found = conn
    .packets()
    .find(|x| match x {
      ServerPacket::ServerMessage(msg) => msg.text == "test-message",
      _ => false,
    })
    .is_some();
  assert!(found, "Server message not found");
}
