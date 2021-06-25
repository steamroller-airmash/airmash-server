use crate::component::*;
use crate::event::EventBoost;
use crate::AirmashWorld;

/// When an internal EventBoost occurs we also need to forward it on so that
/// clients know that it has happened as well.
#[handler]
fn send_boost_packet(event: &EventBoost, game: &mut AirmashWorld) {
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
