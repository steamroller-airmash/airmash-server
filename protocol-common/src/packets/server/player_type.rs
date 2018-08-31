use enums::PlaneType;
use types::Player;

/// A player has switched planes.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct PlayerType {
	pub id: Player,
	#[cfg_attr(features = "serde", serde(rename = "type"))]
	pub ty: PlaneType,
}
