/// Say something in public chat.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chat {
	pub text: String,
}
