/// Send a message to your team.
#[derive(Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct TeamChat {
	pub text: String,
}
