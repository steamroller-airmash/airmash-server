use types::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatTeam {
	pub id: Player,
	pub text: String,
}
