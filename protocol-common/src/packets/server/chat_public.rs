use types::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatPublic {
	pub id: Player,
	pub text: String,
}
