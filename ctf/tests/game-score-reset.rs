use std::time::Duration;

use airmash::component::*;
use airmash::protocol::ServerPacket;
use airmash::resource::Config;
use airmash::test::*;
use airmash_server_ctf::config::{FLAG_NO_REGRAB_TIME, RED_TEAM};
use airmash_server_ctf::resource::GameScores;

#[test]
fn scores_reset_on_game_win() {
  let (mut game, mut mock) = TestGame::new();
  airmash_server_ctf::setup_ctf_server(&mut game);

  let mut conn = mock.open();
  let ent = conn.login("test", &mut game);

  game.resources.write::<Config>().admin_enabled = true;
  game.world.insert_one(ent, Team(RED_TEAM)).unwrap();
  game.run_once();

  let pause_time = FLAG_NO_REGRAB_TIME + Duration::from_secs(1);

  // 3 caps by red team
  conn.send_command("teleport", "0 blue-flag");
  game.run_for(pause_time);
  conn.send_command("teleport", "0 red-flag");
  game.run_once();
  conn.send_command("teleport", "0 blue-flag");
  game.run_for(pause_time);
  conn.send_command("teleport", "0 red-flag");
  game.run_once();
  conn.send_command("teleport", "0 blue-flag");
  game.run_for(pause_time);
  conn.send_command("teleport", "0 red-flag");
  game.run_once();

  let last_flag = conn
    .packets()
    .filter_map(|x| match x {
      ServerPacket::GameFlag(flag) => Some(flag),
      _ => None,
    })
    .last()
    .expect("No GameFlag updates were sent");

  println!("{:?}", *game.resources.read::<GameScores>());

  assert_eq!(last_flag.blueteam, 0);
  assert_eq!(last_flag.redteam, 3);

  game.run_for(Duration::from_secs(61));
  game.world.insert_one(ent, Team(RED_TEAM)).unwrap();

  let last_flag = conn
    .packets()
    .filter_map(|x| match x {
      ServerPacket::GameFlag(flag) => Some(flag),
      _ => None,
    })
    .last()
    .expect("No GameFlag updates were sent");

  println!("{:?}", *game.resources.read::<GameScores>());

  assert_eq!(last_flag.blueteam, 0);
  assert_eq!(last_flag.redteam, 0);

  game.run_for(pause_time);
  conn.send_command("teleport", "0 blue-flag");
  game.run_once();
  conn.send_command("teleport", "0 red-flag");
  game.run_count(5);

  let last_flag = conn
    .packets()
    .filter_map(|x| match x {
      ServerPacket::GameFlag(flag) => Some(flag),
      _ => None,
    })
    .last()
    .expect("No GameFlag updates were sent");

  println!("{:?}", *game.resources.read::<GameScores>());

  assert_eq!(last_flag.blueteam, 0);
  assert_eq!(last_flag.redteam, 1);
}
