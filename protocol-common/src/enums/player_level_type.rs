/// Indicate whether a player levelled up, or has
/// just logged in and their level is being communicated
/// to the client.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub enum PlayerLevelType {
	Login = 0,
	LevelUp = 1,
}
