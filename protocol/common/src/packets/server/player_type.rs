use crate::enums::PlaneType;
use crate::types::Player;

/// A player has switched planes.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerType {
	pub id: Player,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: PlaneType,
}
