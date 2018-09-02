impl_try_from_enum! {
	/// Specific identifiers for server custom messages.
	///
	/// TODO: Reverse Engineer
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
	pub enum ServerCustomType {
		/// TODO: Determine if this name is accurate
		CTFWin = 2,
	}
}
