use enums::MobType;
use types::{Mob, Position};

/// Update for powerups
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MobUpdateStationary {
	pub id: Mob,
	#[serde(rename = "type")]
	pub ty: MobType,
	pub pos: Position,
}
