use super::*;

#[allow(dead_code)]
pub struct NoShuffle;

impl ShuffleProvider for NoShuffle {
	fn shuffle(&self, _: Vec<PlayerShuffleInfo>) -> Vec<TeamChangeEntry> {
		vec![]
	}
}
