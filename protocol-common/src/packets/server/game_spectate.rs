use types::Player;

/// Update which player the client is spectating.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct GameSpectate {
	pub id: Player,
}
