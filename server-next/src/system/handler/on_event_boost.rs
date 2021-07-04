use crate::component::*;
use crate::event::EventBoost;
use crate::resource::Config;
use crate::AirmashGame;

/// When an internal EventBoost occurs we also need to forward it on so that
/// clients know that it has happened as well.
#[handler]
fn send_boost_packet(event: &EventBoost, game: &mut AirmashGame) {
  use crate::protocol::server as s;

  let clock = crate::util::get_current_clock(game);

  let mut query = match game
    .world
    .query_one::<(&Position, &Rotation, &Velocity, &Energy, &EnergyRegen)>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some((pos, rot, vel, energy, regen)) = query.get() {
    let packet = s::EventBoost {
      clock,
      id: event.player.id() as _,
      boost: event.boosting,
      pos: pos.0,
      rot: rot.0,
      speed: vel.0,
      energy: energy.0,
      energy_regen: regen.0,
    };

    game.send_to_visible(pos.0, packet);
  }
}

/// When boosting the player has negative regen. We need to set that.
#[handler]
fn set_player_energy_regen(event: &EventBoost, game: &mut AirmashGame) {
  let regen = match game.world.query_one_mut::<&mut EnergyRegen>(event.player) {
    Ok(query) => query,
    Err(_) => return,
  };

  let config = game.resources.read::<Config>();

  if event.boosting {
    regen.0 = crate::consts::PREDATOR_SPECIAL_REGEN;
  } else {
    regen.0 = config.planes.predator.energy_regen;
  }
}
