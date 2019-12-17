use crate::enums::DespawnType;
use crate::types::Mob;

/// A mob despawned
/// This is used when a powerup despawns
/// and when a missile despawns without
/// hitting anything. It does not cause
/// an explosion to be shown at the location.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobDespawn {
	pub id: Mob,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: DespawnType,
}
