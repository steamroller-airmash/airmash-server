use std::time::Duration;

use airmash::component::*;
use airmash::config::GamePrototype;
use airmash::protocol::{DespawnType, ServerPacket};
use airmash::test::TestGame;
use airmash::util::NalgebraExt;
use airmash::Vector2;

#[test]
fn upgrade_despawns_on_time() {
  let mut config = GamePrototype::default();
  config.view_radius = 1000.0;

  let (mut game, mut mock) = TestGame::with_config(config);

  let mut client = mock.open();
  let ent = client.login("test", &mut game);

  game.world.get_mut::<Position>(ent).unwrap().0 = Vector2::zeros();
  let mob = game.spawn_mob(
    MobType::Upgrade,
    Vector2::new(100.0, 100.0),
    Duration::from_secs(5),
  );

  game.run_for(Duration::from_secs(6));

  loop {
    match client.next_packet() {
      Some(ServerPacket::MobDespawn(evt)) => {
        assert_eq!(evt.ty, DespawnType::LifetimeEnded);
        assert_eq!(evt.id as u32, mob.id());
        break;
      }
      Some(_) => (),
      None => panic!("Never recieved MobDespawn packet"),
    }
  }
}
