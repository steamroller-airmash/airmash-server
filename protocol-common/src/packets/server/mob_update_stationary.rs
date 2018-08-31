use enums::MobType;
use types::{Mob, Position};

/// Update for powerups
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct MobUpdateStationary {
	pub id: Mob,
	#[cfg_attr(features = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
}
