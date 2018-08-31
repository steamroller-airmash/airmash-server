use types::{Player, Position};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct PlayerKill {
	pub id: Player,
	pub killer: Option<Player>,
	pub pos: Position,
}
