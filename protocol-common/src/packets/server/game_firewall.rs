use enums::{FirewallStatus, FirewallUpdateType};
use types::Position;

/// Update the "Wall of Fire" in BTR
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct GameFirewall {
	#[cfg_attr(features = "serde", serde(rename = "type"))]
	pub ty: FirewallUpdateType,
	pub status: FirewallStatus,
	pub pos: Position,
	pub radius: f32,
	pub speed: f32,
}
