mod even_shuffle;
mod no_shuffle;
mod random_shuffle;
mod structs;

pub use self::even_shuffle::EvenShuffle;
pub use self::no_shuffle::NoShuffle;
pub use self::random_shuffle::RandomShuffle;

pub use self::structs::{PlayerShuffleInfo, TeamChangeEntry};

pub trait ShuffleProvider {
	fn shuffle(&self, infos: Vec<PlayerShuffleInfo>) -> Vec<TeamChangeEntry>;
}

pub fn get_shuffle() -> Box<ShuffleProvider + Sync + Send> {
	Box::new(EvenShuffle)
}
