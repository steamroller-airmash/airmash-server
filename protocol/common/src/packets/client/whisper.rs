use crate::types::Player;

/// Send a whisper to another player.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Whisper {
	pub id: Player,
	pub text: String,
}
