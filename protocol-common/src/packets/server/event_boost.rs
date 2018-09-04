use types::{Energy, EnergyRegen, Player, Position, Rotation, Velocity};

/// A predator has begun/stopped boosting
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventBoost {
	pub clock: u32,
	pub id: Player,
	pub boost: bool,
	pub pos: Position,
	pub rot: Rotation,
	pub speed: Velocity,
	pub energy: Energy,
	pub energy_regen: EnergyRegen,
}
