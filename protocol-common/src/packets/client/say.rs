/// Say a message in a chat bubble
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Say {
	pub text: String,
}
