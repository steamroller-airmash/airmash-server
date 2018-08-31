use types::Player;

#[derive(Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct ChatSay {
	pub id: Player,
	pub text: String,
}
