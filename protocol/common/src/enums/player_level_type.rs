/// Indicate whether a player levelled up, or has
/// just logged in and their level is being communicated
/// to the client.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Conversions)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PlayerLevelType {
	Login = 0,
	LevelUp = 1,
}

impl Default for PlayerLevelType {
	fn default() -> Self {
		PlayerLevelType::Login
	}
}
