/// Say a message in a chat bubble
#[derive(Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct Say {
	pub text: String,
}
