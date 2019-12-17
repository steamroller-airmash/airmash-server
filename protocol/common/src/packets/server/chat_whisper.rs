use crate::types::Player;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatWhisper {
	pub from: Player,
	pub to: Player,
	pub text: String,
}
