use enums::MobType;
use types::{Accel, Energy, EnergyRegen, Mob, Player, Position, Speed, Velocity};

/// Data on a projectile fired by a plane.
///
/// This is used in the `projectiles` array
/// of the [`PlayerFire`] packet.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerFireProjectile {
	pub id: Mob,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
	pub speed: Velocity,
	pub accel: Accel,
	pub max_speed: Speed,
}

/// Packet for whan a player fires missiles.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerFire {
	pub clock: u32,
	pub id: Player,
	pub energy: Energy,
	pub energy_regen: EnergyRegen,
	pub projectiles: Vec<PlayerFireProjectile>,
}
