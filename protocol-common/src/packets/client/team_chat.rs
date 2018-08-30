/// Send a message to your team.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamChat {
	pub text: String,
}
