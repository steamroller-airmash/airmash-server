use crate::component::*;
use crate::event::PlayerScoreUpdate;
use crate::AirmashGame;

#[handler]
pub fn send_packet(event: &PlayerScoreUpdate, game: &mut AirmashGame) {
  use crate::protocol::server::ScoreUpdate;

  let (score, earnings, upgrades, kills, deaths, _) = match game.world.query_one_mut::<(
    &Score,
    &Earnings,
    &Upgrades,
    &KillCount,
    &DeathCount,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = ScoreUpdate {
    id: event.player.id() as _,
    score: score.0,
    earnings: earnings.0,
    upgrades: upgrades.unused,
    total_deaths: deaths.0,
    total_kills: kills.0,
  };

  game.send_to(event.player, packet);
}
