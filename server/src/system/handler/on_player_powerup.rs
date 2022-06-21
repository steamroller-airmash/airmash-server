use crate::component::{IsPlayer, *};
use crate::event::PlayerPowerup;
use crate::AirmashGame;

#[handler]
fn send_packet(event: &PlayerPowerup, game: &mut AirmashGame) {
  use crate::protocol::server as s;

  if game.world.query_one_mut::<&IsPlayer>(event.player).is_err() {
    return;
  }

  let (duration, ty) = match (event.powerup.duration, event.powerup.server_type) {
    (Some(duration), Some(ty)) => (duration, ty),
    _ => return,
  };

  let duration = duration.as_secs() * 1000 + duration.subsec_millis() as u64;

  game.send_to(
    event.player,
    s::PlayerPowerup {
      duration: duration as u32,
      ty,
    },
  );
}

#[handler]
fn update_effects(event: &PlayerPowerup, game: &mut AirmashGame) {
  let start_time = game.start_time();
  let this_frame = game.this_frame();

  let (duration, ty) = match (event.powerup.duration, event.powerup.server_type) {
    (Some(duration), Some(ty)) => (duration, ty),
    _ => return,
  };

  let (last_update, effects, _) = match game
    .world
    .query_one_mut::<(&mut LastUpdateTime, &mut Effects, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  last_update.0 = start_time;
  effects.set_powerup(ty, this_frame + duration, &event.powerup.effects);
}
