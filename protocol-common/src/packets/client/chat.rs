/// Say something in public chat.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat {
	pub text: String,
}
