use std::time::Duration;

use airmash::component::Team;
use airmash::protocol::{FlagUpdateType, ServerPacket};
use airmash::resource::GameConfig;
use airmash::test::TestGame;
use airmash_server_ctf::config::{FLAG_NO_REGRAB_TIME, RED_TEAM};

#[test]
fn player_with_flag_has_flagspeed_set() {
  let (mut game, mut mock) = TestGame::new();
  airmash_server_ctf::setup_ctf_server(&mut game);

  let mut client = mock.open();
  let entity = client.login("test", &mut game);

  game.resources.write::<GameConfig>().admin_enabled = true;
  game.world.insert_one(entity, Team(RED_TEAM)).unwrap();
  game.run_for(FLAG_NO_REGRAB_TIME + Duration::from_secs(1));

  client.send_command("teleport", "0 blue-flag");
  // Drain all packets
  let _ = client.packets().count();
  game.run_for(Duration::from_secs(3));

  let has_picked_up_flag = client
    .packets()
    .filter_map(|p| match p {
      ServerPacket::GameFlag(p) => Some(p),
      _ => None,
    })
    .any(|p| p.flag == 1 && p.ty == FlagUpdateType::Carrier);
  let has_flagspeed_update = client
    .packets()
    .filter_map(|p| match p {
      ServerPacket::PlayerUpdate(p) => Some(p),
      _ => None,
    })
    .any(|p| p.keystate.flagspeed);

  assert!(has_picked_up_flag);
  assert!(has_flagspeed_update);
}
