use types::Player;

/// Vote to mute a player
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct VoteMute {
	id: Player,
}
