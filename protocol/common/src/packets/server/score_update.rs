use crate::types::{Player, Score};

/// Stats for the current player
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreUpdate {
	pub id: Player,
	pub score: Score,
	pub earnings: Score,
	/// The number of unused upgrades that the player has.
	pub upgrades: u16,
	pub total_kills: u32,
	pub total_deaths: u32,
}
