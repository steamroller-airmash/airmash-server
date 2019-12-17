/// Send a message to your team.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TeamChat {
	pub text: String,
}
