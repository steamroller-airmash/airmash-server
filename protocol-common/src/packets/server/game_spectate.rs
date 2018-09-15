use types::Player;

/// Update which player the client is spectating.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameSpectate {
	pub id: Player,
}
