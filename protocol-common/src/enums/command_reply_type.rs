impl_try_from_enum! {
	/// TODO: Reverse engineer
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
	pub enum CommandReplyType {}
}
