use crate::enums::MobType;
use crate::types::{Health, HealthRegen, Mob, Player, Position};

/// Data on a player that has been hit by a shot
/// fired by another player.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerHitPlayer {
	pub id: Player,
	pub health: Health,
	pub health_regen: HealthRegen,
}

/// Event for when players have been hit by a missile.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerHit {
	pub id: Mob,
	#[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
	pub owner: Player,
	pub players: Vec<PlayerHitPlayer>,
}
