use specs::DenseVecStorage;

impl_try_from_enum! {
	/// Indicates the type of entity that just
	/// went outside of the player's horizon.
	///
	/// TODO: Complete reverse engineering this.
	/// NOTE: The values here aren't in any way
	/// certain and should be verified before
	/// relying upon them.
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum LeaveHorizonType {
		Player = 0,
		Mob = 1,
	}
}
