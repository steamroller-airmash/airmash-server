use std::convert::TryInto;

use crate::component::*;
use crate::AirmashGame;
use crate::{event::PacketEvent, protocol::client::ScoreDetailed};

pub fn register_all(game: &mut AirmashGame) {
  game.register(respond_to_packet);
}

fn respond_to_packet(event: &PacketEvent<ScoreDetailed>, game: &mut AirmashGame) {
  use crate::protocol::server::{ScoreDetailedCTF, ScoreDetailedCTFEntry};

  let mut scores = Vec::new();
  let query = game
    .world
    .query_mut::<(
      &Level,
      &Captures,
      &Score,
      &KillCount,
      &DeathCount,
      &TotalDamage,
      &PlayerPing,
    )>()
    .with::<IsPlayer>();
  for (player, (level, captures, score, kills, deaths, damage, ping)) in query {
    scores.push(ScoreDetailedCTFEntry {
      id: player.id() as _,
      level: level.0,
      captures: captures.0.try_into().unwrap_or(u16::MAX),
      score: score.0,
      kills: kills.0.try_into().unwrap_or(u16::MAX),
      deaths: deaths.0.try_into().unwrap_or(u16::MAX),
      damage: damage.0,
      ping: ping.0,
    });
  }

  let packet = ScoreDetailedCTF { scores };
  game.send_to(event.entity, packet);
}
