use crate::component::IsPlayer;
use crate::component::*;
use crate::event::PlayerPowerup;
use crate::resource::{StartTime, ThisFrame};
use crate::AirmashWorld;

#[handler]
fn send_packet(event: &PlayerPowerup, game: &mut AirmashWorld) {
  use crate::protocol::server as s;

  if let Err(_) = game.world.query_one_mut::<&IsPlayer>(event.player) {
    return;
  }

  let duration = event.duration.as_secs() * 1000 + event.duration.subsec_millis() as u64;

  game.send_to(
    event.player,
    s::PlayerPowerup {
      duration: duration as u32,
      ty: event.ty,
    },
  );
}

#[handler]
fn update_fields(event: &PlayerPowerup, game: &mut AirmashWorld) {
  let start_time = game.resources.read::<StartTime>().0;
  let this_frame = game.resources.read::<ThisFrame>().0;

  let (last_update, powerup, _) = match game
    .world
    .query_one_mut::<(&mut LastUpdateTime, &mut Powerup, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  last_update.0 = start_time;
  *powerup = Powerup::new(event.ty, this_frame + event.duration);
}
