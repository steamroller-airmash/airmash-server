use specs::DenseVecStorage;

impl_try_from_enum! {
	/// TODO: Reverse engineer
	///
	/// This might be just [`PlaneType`][0] instead.
	///
	/// [0]: struct.PlaneType.html
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum UpgradeType {}
}
