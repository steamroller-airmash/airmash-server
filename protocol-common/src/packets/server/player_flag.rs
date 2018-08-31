use enums::FlagCode;
use types::Player;

/// Packet for when a player changes their flag.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerFlag {
	pub id: Player,
	pub flag: FlagCode,
}
