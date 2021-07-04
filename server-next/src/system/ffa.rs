//! Event handlers for the FFA scoreboard.

use std::convert::TryInto;

use crate::protocol::client::ScoreDetailed;
use crate::{
  component::*,
  event::{PlayerHit, PlayerJoin},
};
use crate::{event::PacketEvent, AirmashGame};

#[derive(Copy, Clone, Debug)]
pub struct TotalDamage(f32);

pub fn register_all(game: &mut AirmashGame) {
  game.register(respond_to_packet);
  game.register(add_damage_on_join);
  game.register(update_damage);
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
      ping: ping.0,
    });
  }

  game.send_to(event.entity, ScoreDetailedFFA { scores });
}

fn add_damage_on_join(event: &PlayerJoin, game: &mut AirmashGame) {
  let _ = game.world.insert_one(event.player, TotalDamage(0.0));
}

fn update_damage(event: &PlayerHit, game: &mut AirmashGame) {
  let attacker = match event.attacker {
    Some(attacker) => attacker,
    None => return,
  };

  let (damage, _) = match game
    .world
    .query_one_mut::<(&mut TotalDamage, &IsPlayer)>(attacker)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  damage.0 += event.damage;
}
