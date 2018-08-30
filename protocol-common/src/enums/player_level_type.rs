/// Indicate whether a player levelled up, or has
/// just logged in and their level is being communicated
/// to the client.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayeLevelType {
	Login = 0,
	LevelUp = 1,
}
