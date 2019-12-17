use crate::enums::FlagUpdateType;
use crate::types::{Flag, Player, Position};

/// Update position of flag in CTF
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameFlag {
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: FlagUpdateType,
	pub flag: Flag,
	pub id: Option<Player>,
	pub pos: Position,
	/// Blue team score
	pub blueteam: u8,
	/// Red team score
	pub redteam: u8,
}
