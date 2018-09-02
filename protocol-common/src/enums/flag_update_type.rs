impl_try_from_enum! {
	/// Flag update type
	///
	/// Used to indicate whether the flag is now being
	/// carried by a player or whether the update
	/// sets the position of the flag directly.
	///
	/// Used in:
	/// - TODO
	///
	/// Implementors Note: This had a `TODO: rev-eng`
	/// comment on it but it doesn't seem to be missing
	/// any values. It might be worth doing some more
	/// looking to see if anything turns up here.
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
	pub enum FlagUpdateType {
		Position = 1,
		Carrier = 2,
	}
}
