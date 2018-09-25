use super::*;

use rand;

#[allow(dead_code)]
pub struct RandomShuffle;

impl ShuffleProvider for RandomShuffle {
	fn shuffle(&self, infos: Vec<PlayerShuffleInfo>) -> Vec<TeamChangeEntry> {
		infos
			.into_iter()
			.filter_map(|info| {
				if rand::random() {
					Some(info.into())
				} else {
					None
				}
			}).collect()
	}
}
