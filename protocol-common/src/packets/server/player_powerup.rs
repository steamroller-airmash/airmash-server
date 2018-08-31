use enums::PowerupType;

/// A player picked up a powerup
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct PlayerPowerup {
	#[cfg_attr(features = "serde", serde(rename = "type"))]
	pub ty: PowerupType,
	// Maybe make this a Duration?
	duration: u32,
}
