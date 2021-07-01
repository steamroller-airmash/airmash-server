use crate::component::*;
use crate::event::EventStealth;
use crate::resource::StartTime;
use crate::AirmashWorld;

#[handler(priority = crate::priority::MEDIUM)]
fn update_player_state(event: &EventStealth, game: &mut AirmashWorld) {
  let this_frame = game.this_frame();
  let start_time = game.resources.read::<StartTime>().0;

  let (active, last_special, last_update, keystate, _) = match game.world.query_one_mut::<(
    &mut SpecialActive,
    &mut LastSpecialTime,
    &mut LastUpdateTime,
    &mut KeyState,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  active.0 = event.stealthed;
  last_special.0 = this_frame;
  last_update.0 = start_time;
  keystate.stealthed = event.stealthed;
}

#[handler]
fn send_packet(event: &EventStealth, game: &mut AirmashWorld) {
  use crate::protocol::server as s;

  let (&pos, energy, regen, _) = match game
    .world
    .query_one_mut::<(&Position, &Energy, &EnergyRegen, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = s::EventStealth {
    id: event.player.id() as _,
    state: event.stealthed,
    energy: energy.0,
    energy_regen: regen.0,
  };

  if event.stealthed {
    game.send_to_visible(pos.0, packet);
  } else {
    game.send_to(event.player, packet);
  }
}
