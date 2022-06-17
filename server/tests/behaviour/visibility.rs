use std::time::Duration;

use airmash_protocol::{KeyCode, MobType, ServerPacket};
use airmash_server::component::Position;
use airmash_server::resource::Config;
use airmash_server::Vector2;
use nalgebra::vector;
use server::test::TestGame;

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
  let (mut game, mut mock) = TestGame::new();

  let mut client1 = mock.open();
  let mut client2 = mock.open();

  let ent1 = client1.login("test-1", &mut game);
  let ent2 = client2.login("test-2", &mut game);

  let view_radius = game.resources.read::<Config>().view_radius;

  game.world.get_mut::<Position>(ent2).unwrap().0 = vector![0.0, -view_radius + 1.0];
  game.run_count(60);

  client2.send_key(KeyCode::Fire, true);

  game.run_count(5);

  while let Some(packet) = client1.next_packet() {
    match packet {
      ServerPacket::EventLeaveHorizon(evt) => {
        assert_ne!(evt.id as u32, ent1.id());
        assert_ne!(evt.id as u32, ent2.id());

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
  let (mut game, mut mock) = TestGame::new();

  let offset = {
    let mut config = game.resources.write::<Config>();
    config.planes.predator.missile_offset.x = 500.0;
    config.view_radius = 100.0;

    config.planes.predator.missile_offset.x
  };

  let mut client1 = mock.open();
  let mut client2 = mock.open();

  let ent1 = client1.login("test-1", &mut game);
  let ent2 = client2.login("test-2", &mut game);

  let id1 = ent1.id() as u16;
  let id2 = ent2.id() as u16;

  // There is a mountain at (x: -252, y: -1504) with r = 60
  let object = vector![-252.0, -1504.0];

  let pos = vector![object.x, object.y - offset];

  game.world.get_mut::<Position>(ent1).unwrap().0 = pos;
  game.world.get_mut::<Position>(ent2).unwrap().0 = pos;

  client1.send_key(KeyCode::Fire, true);
  game.run_count(5);

  loop {
    match client2.next_packet() {
      Some(ServerPacket::PlayerFire(evt)) => {
        assert_eq!(evt.id, id1);
        break;
      }
      Some(_) => (),
      None => panic!("Never received PlayerFire packet"),
    }
  }

  loop {
    match client2.next_packet() {
      Some(ServerPacket::MobDespawnCoords(evt)) => {
        assert_ne!(evt.id, id1);
        assert_ne!(evt.id, id2);
        break;
      }
      Some(ServerPacket::EventLeaveHorizon(evt)) => {
        assert_ne!(evt.id, id1);
        assert_ne!(evt.id, id2);
        break;
      }
      Some(_) => (),
      None => panic!("Never recieved MobDespawnCoords or EventLeaveHorizon packet"),
    }
  }
}

#[test]
fn out_of_visibility_mob() {
  let (mut game, mut mock) = TestGame::new();

  game.resources.write::<Config>().view_radius = 500.0;

  let mut client = mock.open();
  client.login("test1", &mut game);

  let mob = game.spawn_mob(
    airmash_protocol::MobType::Upgrade,
    Vector2::new(200.0, -550.0),
    Duration::from_secs(1000),
  );

  game.run_once();

  client.send_key(KeyCode::Up, true);
  game.run_for(Duration::from_secs(2));

  loop {
    match client.next_packet() {
      Some(ServerPacket::MobUpdateStationary(evt)) => {
        assert_eq!(evt.id as u32, mob.id());
        break;
      }
      Some(_) => (),
      None => panic!("Never received MobUpdateStationary packet"),
    }
  }
}

#[test]
fn edge_of_visibility_mob() {
  let (mut game, mut mock) = TestGame::new();

  let mut client = mock.open();
  let player = client.login("test", &mut game);

  game.resources.write::<Config>().view_radius = 500.0;
  game.world.get_mut::<Position>(player).unwrap().0 = Vector2::zeros();

  game.run_count(5);

  let mob = game.spawn_mob(
    MobType::Upgrade,
    Vector2::new(500.0, 0.0),
    Duration::from_secs(5),
  );

  game.world.get_mut::<Position>(player).unwrap().x = 5000.0;

  game.run_count(5);

  let evt = client
    .packets()
    .find_map(|p| match p {
      ServerPacket::MobUpdateStationary(p) => Some(p),
      _ => None,
    })
    .unwrap_or_else(|| panic!("Never recieved ModUpdateStationary packet"));

  assert_eq!(evt.id as u32, mob.id());

  let evt = client
    .packets()
    .find_map(|p| match p {
      ServerPacket::EventLeaveHorizon(p) => Some(p),
      _ => None,
    })
    .unwrap_or_else(|| panic!("Never recieved EventLeaveHorizon packet"));

  assert_eq!(evt.id as u32, mob.id());
}
