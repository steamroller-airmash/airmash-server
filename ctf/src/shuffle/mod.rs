use crate::config::{BLUE_TEAM, RED_TEAM};
use airmash::{component::*, AirmashGame, Entity};

pub struct TeamChangeEntry {
  pub player: Entity,
  pub team: u16,
}

pub enum ShuffleType {
  AlternatingScore,
}

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

pub fn shuffle(game: &mut AirmashGame, ty: ShuffleType) -> Vec<TeamChangeEntry> {
  match ty {
    ShuffleType::AlternatingScore => alternating_score_shuffle(game),
  }
}
