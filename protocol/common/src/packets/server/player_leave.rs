use crate::types::Player;

/// Packet for when a player leaves.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerLeave {
	pub id: Player,
}
