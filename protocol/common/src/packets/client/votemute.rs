use crate::types::Player;

/// Vote to mute a player
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VoteMute {
	pub id: Player,
}
