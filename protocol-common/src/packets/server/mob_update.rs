use enums::MobType;
use types::{Accel, Mob, Position, Speed, Velocity};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobUpdate {
	pub clock: u32,
	pub id: Mob,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
	pub speed: Velocity,
	pub accel: Accel,
	pub max_speed: Speed,
}
