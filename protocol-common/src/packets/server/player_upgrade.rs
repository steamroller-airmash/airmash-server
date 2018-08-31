use enums::UpgradeType;
use types::Upgrades;

/// A player has upgraded themselves.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerUpgrade {
	pub upgrades: Upgrades,
	/// Is this actually PlaneType?
	#[serde(rename = "type")]
	pub ty: UpgradeType,
	pub speed: u8,
	pub defense: u8,
	pub energy: u8,
	pub missile: u8,
}
