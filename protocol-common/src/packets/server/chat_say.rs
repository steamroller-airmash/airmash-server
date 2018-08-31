use types::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatSay {
	pub id: Player,
	pub text: String,
}
