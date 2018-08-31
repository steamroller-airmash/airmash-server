use types::Player;

/// Update which player the client is spectating.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GameSpectate {
	pub id: Player,
}
