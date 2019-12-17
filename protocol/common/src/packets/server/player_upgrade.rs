use crate::enums::UpgradeType;

/// A player has upgraded themselves.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerUpgrade {
	pub upgrades: u16,
	/// Is this actually PlaneType?
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: UpgradeType,
	pub speed: u8,
	pub defense: u8,
	pub energy: u8,
	pub missile: u8,
}
