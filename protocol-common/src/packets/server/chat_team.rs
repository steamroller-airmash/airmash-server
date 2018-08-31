use types::Player;

#[derive(Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct ChatTeam {
	pub id: Player,
	pub text: String,
}