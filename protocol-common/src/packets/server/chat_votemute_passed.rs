use types::Player;

/// A player has been votemuted
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChatVoteMutePassed {
	pub id: Player,
}
