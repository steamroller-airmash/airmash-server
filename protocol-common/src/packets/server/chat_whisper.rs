use types::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatWhisper {
	pub from: Player,
	pub to: Player,
	pub text: String,
}
