use types::Player;

/// Vote to mute a player
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct VoteMute {
	id: Player,
}
