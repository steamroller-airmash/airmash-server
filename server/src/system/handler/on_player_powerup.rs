use crate::component::{IsPlayer, *};
use crate::event::PlayerPowerup;
use crate::protocol::server as s;
use crate::AirmashGame;

#[handler]
fn send_packet(event: &PlayerPowerup, game: &mut AirmashGame) {
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

#[handler(priority = crate::priority::HIGH)]
fn update_player_upgrades(event: &PlayerPowerup, game: &mut AirmashGame) {
  let num: u16 = event.powerup.upgrade_count();
  if num == 0 {
    return;
  }

  let (upgrades, prev, _) = match game
    .world
    .query_one_mut::<(&mut Upgrades, &mut PrevUpgrades, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  upgrades.unused += num;
  prev.0 = *upgrades;
}

#[handler]
fn send_player_upgrade(event: &PlayerPowerup, game: &mut AirmashGame) {
  let (upgrades, score, earnings, kills, deaths, _) = match game.world.query_one_mut::<(
    &Upgrades,
    &Score,
    &Earnings,
    &KillCount,
    &DeathCount,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = s::ScoreUpdate {
    id: event.player.id() as _,
    upgrades: upgrades.unused,
    score: score.0,
    earnings: earnings.0,
    total_kills: kills.0,
    total_deaths: deaths.0,
  };
  game.send_to(event.player, packet);
}
