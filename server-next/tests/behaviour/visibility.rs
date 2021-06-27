use airmash_protocol::ServerPacket;
use airmash_server::component::Position;
use airmash_server::resource::Config;
use nalgebra::vector;
use server::protocol::client as c;

/// If a player is at the very edge of another player's horizon then
/// LeaveHorizon packets may not be sent correctly because the missile might be
/// spawned outside of their horizon.
///
/// However, if this happens then the first player will still get the PlayerFire
/// packet but not the EventLeaveHorizon packet. This results in zombie missiles
/// on the client. This tests that the fix for this is valid and that we don't
/// get zombie missiles.
#[test]
fn out_of_visibility_missiles_properly_deleted() {
  let (mut game, mut mock) = crate::utils::create_mock_server();

  let (conn1, mut rx1) = mock.open();
  let (conn2, mut rx2) = mock.open();

  mock.send(conn1, crate::utils::create_login_packet("test-1"));
  mock.send(conn2, crate::utils::create_login_packet("test-2"));

  game.run_once();

  let id1 = crate::utils::get_login_id(&mut rx1);
  let id2 = crate::utils::get_login_id(&mut rx2);

  let ent2 = game.get_entity_by_id(id2).unwrap();

  let view_radius = {
    let config = game.resources.read::<Config>();
    config.view_radius
  };

  game.world.get_mut::<Position>(ent2).unwrap().0 = vector![0.0, -view_radius + 1.0];
  game.run_count(60);

  mock.send(
    conn2,
    c::Key {
      key: airmash_protocol::KeyCode::Fire,
      seq: 0,
      state: true,
    },
  );

  game.run_count(5);

  while let Some(packet) = rx1.next_packet() {
    match packet {
      ServerPacket::EventLeaveHorizon(evt) => {
        assert_ne!(evt.id, id1);
        assert_ne!(evt.id, id2);

        return;
      }
      _ => (),
    }
  }

  assert!(false, "No leavehorizon packet found")
}

/// If a missile despawns within the same frame as it is spawned then it might
/// not be properly communicated to players that received the PlayerFire packet
/// but which have the missile out of visible range.
///
/// This test verifies that in that case the player either receives a
/// MobDespawnCoords packet or a EventLeaveHorizon packet.
#[test]
fn out_of_visibility_collision() {
  let (mut game, mut mock) = crate::utils::create_mock_server();

  let (conn1, mut rx1) = mock.open();
  let (conn2, mut rx2) = mock.open();

  mock.send(conn1, crate::utils::create_login_packet("test-1"));
  mock.send(conn2, crate::utils::create_login_packet("test-2"));

  game.run_once();

  let id1 = crate::utils::get_login_id(&mut rx1);
  let id2 = crate::utils::get_login_id(&mut rx2);

  let ent1 = game.get_entity_by_id(id1).unwrap();
  let ent2 = game.get_entity_by_id(id2).unwrap();

  let view_radius = {
    let config = game.resources.read::<Config>();
    config.view_radius
  };

  // There is a mountain at (x: -252, y: -1504) with r = 60
  let object = vector![-252.0, -1504.0];
  let radius = 60.0;

  let pos1 = vector![object.x, object.y - radius - 30.0];
  let pos2 = vector![object.x, pos1.y - view_radius];

  game.world.get_mut::<Position>(ent1).unwrap().0 = pos1;
  game.world.get_mut::<Position>(ent2).unwrap().0 = pos2;

  mock.send(
    conn1,
    c::Key {
      key: airmash_protocol::KeyCode::Fire,
      seq: 0,
      state: true,
    },
  );
  game.run_count(1);
  mock.send(
    conn1,
    c::Key {
      key: airmash_protocol::KeyCode::Fire,
      seq: 1,
      state: false,
    },
  );

  game.run_count(5);

  loop {
    match rx2.next_packet() {
      Some(ServerPacket::PlayerFire(evt)) => {
        assert_eq!(evt.id, id1);
        break;
      }
      Some(_) => (),
      None => panic!("Never received PlayerFire packet"),
    }
  }

  loop {
    match rx2.next_packet() {
      Some(ServerPacket::MobDespawnCoords(evt)) => {
        assert_ne!(evt.id, id1);
        assert_ne!(evt.id, id2);
        return;
      }
      Some(ServerPacket::EventLeaveHorizon(evt)) => {
        assert_ne!(evt.id, id1);
        assert_ne!(evt.id, id2);
        return;
      }
      Some(_) => (),
      None => panic!("Never recieved MobDespawnCoords or PlayerFire packet"),
    }
  }
}
