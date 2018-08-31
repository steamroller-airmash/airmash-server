use enums::MobType;
use types::Mob;

/// A mob despawned
/// This is used when a powerup despawns
/// and when a missile despawns without
/// hitting anything. It does not cause
/// an explosion to be shown at the location.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MobDespawn {
	pub id: Mob,
	#[serde(rename = "type")]
	pub ty: MobType,
}
