use types::{Level, Player, Score};

/// Per-player data for detailed (tab) menu in FFA.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedFFAEntry {
	pub id: Player,
	pub level: Level,
	pub score: Score,
	pub kills: u16,
	pub deaths: u16,
	pub damage: f32,
	pub ping: u16,
}

/// Detailed score menu (tab) data for FFA.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedFFA {
	pub scores: Vec<ScoreDetailedFFAEntry>,
}
