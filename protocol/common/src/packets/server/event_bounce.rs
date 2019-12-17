use crate::types::{Player, Position, Rotation, ServerKeyState, Velocity};

/// A player has run into a wall
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventBounce {
	pub clock: u32,
	pub id: Player,
	pub keystate: ServerKeyState,
	pub pos: Position,
	pub rot: Rotation,
	pub speed: Velocity,
}
