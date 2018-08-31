use types::{Player, Position};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerKill {
	pub id: Player,
	pub killer: Option<Player>,
	pub pos: Position,
}
