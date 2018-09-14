use specs::DenseVecStorage;

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
	/// any values.
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum FlagUpdateType {
		Position = 1,
		Carrier = 2,
	}
}
