use types::Player;

/// Packet for when a player leaves.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct PlayerLeave {
	pub id: Player,
}
