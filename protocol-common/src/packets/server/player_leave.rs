use types::Player;

/// Packet for when a player leaves.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerLeave {
	pub id: Player,
}
