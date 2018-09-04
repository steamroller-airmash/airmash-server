use types::Player;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatSay {
	pub id: Player,
	pub text: String,
}
