use types::{Level, Player, Score};

/// Per-player data for detailed (tab) menu in BTR.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedBTREntry {
	pub id: Player,
	pub level: Level,
	pub alive: bool,
	pub wins: u16,
	pub score: Score,
	pub kills: u16,
	pub deaths: u16,
	pub damage: f32,
	pub ping: u16,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedBTR {
	pub scores: Vec<ScoreDetailedBTREntry>,
}
