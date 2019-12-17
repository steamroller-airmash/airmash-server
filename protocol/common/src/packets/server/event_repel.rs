use crate::enums::MobType;
use crate::types::{
	Accel, Energy, EnergyRegen, Health, HealthRegen, Mob, Player, Position, Rotation,
	ServerKeyState, Speed, Velocity,
};

/// A player has been repelled by a goliath.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventRepelPlayer {
	pub id: Player,
	pub keystate: ServerKeyState,
	pub pos: Position,
	pub rot: Rotation,
	pub speed: Velocity,
	pub energy: Energy,
	pub energy_regen: EnergyRegen,
	pub health: Health,
	pub health_regen: HealthRegen,
}

/// A projectile has been repelled by a goliath
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventRepelMob {
	pub id: Mob,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
	pub speed: Velocity,
	pub accel: Accel,
	pub max_speed: Speed,
}

/// Event triggered when something (player or missile)
/// is deflected by a goliath repel.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventRepel {
	pub clock: u32,
	pub id: Player,
	pub pos: Position,
	pub rot: Rotation,
	pub speed: Velocity,
	pub energy: Energy,
	pub energy_regen: EnergyRegen,
	pub players: Vec<EventRepelPlayer>,
	pub mobs: Vec<EventRepelMob>,
}
