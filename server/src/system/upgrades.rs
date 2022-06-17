use crate::component::{IsPlayer, PrevUpgrades, Upgrades};
use crate::protocol::server::PlayerUpgrade;
use crate::protocol::UpgradeType;
use crate::AirmashGame;

pub fn update(game: &mut AirmashGame) {
  send_updates_for_outdated(game);
}

fn send_updates_for_outdated(game: &mut AirmashGame) {
  let mut query = game
    .world
    .query::<(&Upgrades, &mut PrevUpgrades)>()
    .with::<IsPlayer>();
  for (player, (upgrades, prev)) in &mut query {
    if *upgrades == prev.0 {
      continue;
    }

    let packet = PlayerUpgrade {
      upgrades: upgrades.unused,
      ty: UpgradeType::None,
      speed: upgrades.speed,
      defense: upgrades.defense,
      energy: upgrades.energy,
      missile: upgrades.missile,
    };

    if upgrades.speed != prev.0.speed {
      game.force_update(player);
    }

    game.send_to(player, packet);
    prev.0 = *upgrades;
  }
}
