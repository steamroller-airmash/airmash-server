/// Say something in public chat.
#[derive(Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct Chat {
	pub text: String,
}
