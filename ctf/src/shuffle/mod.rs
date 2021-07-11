use crate::config::{BLUE_TEAM, RED_TEAM};
use airmash::{component::*, AirmashGame, Entity};
use rand::prelude::SliceRandom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TeamChangeEntry {
  pub player: Entity,
  pub team: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
#[allow(dead_code)]
pub enum ShuffleType {
  AlternatingScore,
  AlternatingEarnings,
  EvenRandom,
}

/// A shuffle that alternates players in order of current score
pub fn alternating_score_shuffle(game: &mut AirmashGame) -> Vec<TeamChangeEntry> {
  let mut players = game
    .world
    .query_mut::<&Score>()
    .with::<IsPlayer>()
    .into_iter()
    .map(|(player, score)| (player, score.0))
    .collect::<Vec<_>>();

  let start = if rand::random() { 0 } else { 1 };
  let teams = [BLUE_TEAM, RED_TEAM];
  players.sort_unstable_by_key(|p| p.1);

  players
    .into_iter()
    .enumerate()
    .filter_map(|(index, (player, _))| {
      let old_team = game.world.get::<Team>(player).ok()?.0;
      let new_team = teams[(index + start) % 2];

      match old_team == new_team {
        true => None,
        false => Some(TeamChangeEntry {
          player,
          team: new_team,
        }),
      }
    })
    .collect()
}

/// A shuffle that alternates players in order of total earnings
pub fn alternating_earnings_shuffle(game: &mut AirmashGame) -> Vec<TeamChangeEntry> {
  let mut players = game
    .world
    .query_mut::<&Earnings>()
    .with::<IsPlayer>()
    .into_iter()
    .map(|(player, score)| (player, score.0))
    .collect::<Vec<_>>();

  let start = if rand::random() { 0 } else { 1 };
  let teams = [BLUE_TEAM, RED_TEAM];
  players.sort_unstable_by_key(|p| p.1);

  players
    .into_iter()
    .enumerate()
    .filter_map(|(index, (player, _))| {
      let old_team = game.world.get::<Team>(player).ok()?.0;
      let new_team = teams[(index + start) % 2];

      match old_team == new_team {
        true => None,
        false => Some(TeamChangeEntry {
          player,
          team: new_team,
        }),
      }
    })
    .collect()
}

/// A random shuffle where each team ends up with the same number of players
pub fn even_random_shuffle(game: &mut AirmashGame) -> Vec<TeamChangeEntry> {
  let mut players = game
    .world
    .query_mut::<()>()
    .with::<IsPlayer>()
    .into_iter()
    .map(|(player, _)| player)
    .collect::<Vec<_>>();

  let teams = if rand::random() {
    [BLUE_TEAM, RED_TEAM]
  } else {
    [RED_TEAM, BLUE_TEAM]
  };
  let half = players.len() / 2;

  players.shuffle(&mut rand::thread_rng());

  players
    .into_iter()
    .enumerate()
    .filter_map(|(index, player)| {
      let old_team = game.world.get::<Team>(player).ok()?.0;
      let new_team = teams[if index < half { 0 } else { 1 }];

      match old_team == new_team {
        true => None,
        false => Some(TeamChangeEntry {
          player,
          team: new_team,
        }),
      }
    })
    .collect()
}

pub fn shuffle(game: &mut AirmashGame, ty: ShuffleType) -> Vec<TeamChangeEntry> {
  match ty {
    ShuffleType::AlternatingScore => alternating_score_shuffle(game),
    ShuffleType::AlternatingEarnings => alternating_earnings_shuffle(game),
    ShuffleType::EvenRandom => even_random_shuffle(game),
  }
}
