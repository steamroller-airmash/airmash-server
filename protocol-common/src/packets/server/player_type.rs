use enums::PlaneType;
use types::Player;

/// A player has switched planes.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerType {
	pub id: Player,
	#[serde(rename = "type")]
	pub ty: PlaneType,
}
