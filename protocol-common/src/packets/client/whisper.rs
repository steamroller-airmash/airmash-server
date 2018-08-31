use types::Player;

/// Send a whisper to another player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Whisper {
	pub id: Player,
	pub text: String,
}
