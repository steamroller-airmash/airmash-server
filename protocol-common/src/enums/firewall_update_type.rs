use specs::DenseVecStorage;

impl_try_from_enum! {
	/// TODO: Reverse engineer
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum FirewallUpdateType {}
}
