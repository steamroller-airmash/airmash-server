use crate::enums::{FirewallStatus, FirewallUpdateType};
use crate::types::Position;

/// Update the "Wall of Fire" in BTR
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameFirewall {
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: FirewallUpdateType,
	pub status: FirewallStatus,
	pub pos: Position,
	pub radius: f32,
	pub speed: f32,
}
