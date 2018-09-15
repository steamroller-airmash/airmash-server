use enums::PowerupType;

/// A player picked up a powerup
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerPowerup {
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: PowerupType,
	// Maybe make this a Duration?
	pub duration: u32,
}
