use enums::{FirewallStatus, FirewallUpdateType};
use types::Position;

/// Update the "Wall of Fire" in BTR
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GameFirewall {
	#[serde(rename = "type")]
	pub ty: FirewallUpdateType,
	pub status: FirewallStatus,
	pub pos: Position,
	pub radius: f32,
	pub speed: f32,
}
