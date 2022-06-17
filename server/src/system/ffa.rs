//! Event handlers for the FFA scoreboard.

use std::convert::TryInto;

use crate::component::*;
use crate::event::PacketEvent;
use crate::protocol::client::ScoreDetailed;
use crate::AirmashGame;

pub fn register_all(game: &mut AirmashGame) {
  game.register(respond_to_packet);
}

fn respond_to_packet(event: &PacketEvent<ScoreDetailed>, game: &mut AirmashGame) {
  use crate::protocol::server::{ScoreDetailedFFA, ScoreDetailedFFAEntry};

  let mut scores = Vec::new();
  let query = game
    .world
    .query_mut::<(
      &Level,
      &Score,
      &KillCount,
      &DeathCount,
      &TotalDamage,
      &PlayerPing,
    )>()
    .with::<IsPlayer>();
  for (ent, (level, score, kills, deaths, damage, ping)) in query {
    scores.push(ScoreDetailedFFAEntry {
      id: ent.id() as _,
      level: level.0,
      score: score.0,
      kills: kills.0.try_into().unwrap_or(u16::MAX),
      deaths: deaths.0.try_into().unwrap_or(u16::MAX),
      damage: damage.0,
      ping: ping.as_millis().try_into().unwrap_or(u16::MAX),
    });
  }

  game.send_to(event.entity, ScoreDetailedFFA { scores });
}
