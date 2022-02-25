use crate::component::*;
use crate::event::PlayerSpawn;
use crate::resource::GameConfig;
use crate::AirmashGame;

// If GameConfig::always_upgraded is true then we need to stamp over the set of
// upgrades.
#[handler(priority = crate::priority::MEDIUM)]
fn override_player_upgrades(evt: &PlayerSpawn, game: &mut AirmashGame) {
  if !game.resources.read::<GameConfig>().always_upgraded {
    return;
  }

  let upgrades = match game.world.query_one_mut::<&mut Upgrades>(evt.player) {
    Ok(upgrades) => upgrades,
    Err(_) => return,
  };

  upgrades.speed = 5;
  upgrades.defense = 5;
  upgrades.energy = 5;
  upgrades.missile = 5;
}

#[handler]
fn send_upgrade_packet(event: &PlayerSpawn, game: &mut AirmashGame) {
  use crate::protocol::{UpgradeType, server::PlayerUpgrade};

  let upgrades = match game.world.query_one_mut::<&Upgrades>(event.player) {
    Ok(upgrades) => upgrades,
    Err(_) => return,
  };

  let packet = PlayerUpgrade {
    upgrades: upgrades.unused,
    ty: UpgradeType::None,
    speed: upgrades.speed,
    defense: upgrades.defense,
    energy: upgrades.energy,
    missile: upgrades.missile,
  };

  game.send_to(event.player, packet);
}