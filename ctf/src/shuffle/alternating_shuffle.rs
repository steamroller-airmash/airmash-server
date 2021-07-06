use super::*;

use crate::config::{BLUE_TEAM, RED_TEAM};
use rand::random;

/// Shuffle that alternates players between red and
/// blue down the leaderboard.
///
/// The basic algorithm is to sort all the players by
/// score, then to alternate them between each team.
/// (e.g. 1st place on blue, 2nd place on red, etc.)
///
/// Credits to `AES-GCM-128` for the original idea.
#[allow(dead_code)]
pub struct AlternatingShuffle;

impl ShuffleProvider for AlternatingShuffle {
  fn shuffle(&self, infos: Vec<PlayerShuffleInfo>) -> Vec<TeamChangeEntry> {
    let mut values = infos
      .into_iter()
      .filter(|info| info.team == RED_TEAM || info.team == BLUE_TEAM)
      .map(|info| (info.score, info.player, info.team, RED_TEAM))
      .collect::<Vec<_>>();

    values.sort_by(|a, b| a.0.cmp(&b.0));

    let teams;
    if random() {
      teams = [RED_TEAM, BLUE_TEAM];
    } else {
      teams = [BLUE_TEAM, RED_TEAM];
    }

    for i in 0..values.len() {
      values[i].3 = teams[i & 1];
    }

    values
      .into_iter()
      .filter(|(_, _, old, new)| old != new)
      .map(|(_, player, _, new)| TeamChangeEntry {
        player,
        new_team: new,
      })
      .collect()
  }
}
