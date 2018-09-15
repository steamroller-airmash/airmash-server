use specs::DenseVecStorage;

impl_try_from_enum! {
	/// The key that's had it's state changed.
	/// This is only used for client -> server
	/// communication.
	///
	/// It is used in the following packets:
	/// - TODO
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum KeyCode {
		Up = 1,
		Down = 2,
		Left = 3,
		Right = 4,
		Fire = 5,
		Special = 6,
	}
}
