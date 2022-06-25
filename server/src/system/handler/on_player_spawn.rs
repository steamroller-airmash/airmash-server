use crate::component::*;
use crate::event::{PlayerPowerup, PlayerSpawn};
use crate::resource::{Config, GameConfig};
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
fn give_spawn_shield(event: &PlayerSpawn, game: &mut AirmashGame) {
  let proto = game
    .resources
    .read::<Config>()
    .powerups
    .get("spawn-shield")
    .copied();

  if let Some(proto) = proto {
    game.dispatch(PlayerPowerup {
      player: event.player,
      powerup: proto,
    });
  }
}
