use specs::DenseVecStorage;

impl_try_from_enum! {
	/// Type specifier for server banner messages.
	///
	/// TODO: Reverse engineer
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum ServerMessageType {
		TimeToGameStart = 1,
		/// TODO: Verify the value of this one
		Flag = 2,
		/// New Type, used by this server for shutdown message
		/// (once they work)
		Shutdown = 15,
		/// New Type, used by this server for banner messages
		/// on player join.
		Banner = 16,
	}
}
