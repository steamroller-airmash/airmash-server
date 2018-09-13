use enums::PlayerLevelType;
use types::{Level, Player};

/// Assign a level to a player. Either the player
/// levelled up, or the server is updating their
/// level for all clients.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerLevel {
	pub id: Player,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: PlayerLevelType,
	pub level: Level,
}
