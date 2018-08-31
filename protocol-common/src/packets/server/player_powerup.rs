use enums::PowerupType;

/// A player picked up a powerup
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerPowerup {
	#[serde(rename = "type")]
	pub ty: PowerupType,
	// Maybe make this a Duration?
	duration: u32,
}
