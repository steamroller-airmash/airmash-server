use types::Player;

/// A player has been votemuted
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatVoteMutePassed {
	pub id: Player,
}
