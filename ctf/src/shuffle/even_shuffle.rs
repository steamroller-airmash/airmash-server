use super::*;

use config::{BLUE_TEAM, RED_TEAM};
use rand::{random, thread_rng, Rng};

#[allow(dead_code)]
pub struct EvenShuffle;

impl ShuffleProvider for EvenShuffle {
	fn shuffle(&self, infos: Vec<PlayerShuffleInfo>) -> Vec<TeamChangeEntry> {
		let mut values = infos
			.into_iter()
			// If extra teams are implemented for spectators
			// then we don't want them to affect the shuffle
			.filter(|info| info.team == RED_TEAM || info.team == BLUE_TEAM)
			.map(|info| (info.player, info.team, RED_TEAM))
			.collect::<Vec<_>>();

		thread_rng().shuffle(&mut values[..]);

		for i in 0..(values.len() / 2) {
			values[i].2 = BLUE_TEAM;
		}

		if values.len() % 2 != 0 && random() {
			let idx = values.len() / 2;
			values[idx].2 = BLUE_TEAM;
		}

		values
			.into_iter()
			.filter(|(_, old, new)| old != new)
			.map(|(player, _, new)| TeamChangeEntry {
				player,
				new_team: new,
			}).collect()
	}
}
