use crate::enums::{FlagCode, PlaneType, PlayerStatus};
use crate::types::{Player, Position, Rotation, Team, Upgrades};

use std::borrow::Cow;

/// Data for a newly-joined player.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerNew<'data> {
    pub id: Player,
    pub status: PlayerStatus,
    pub name: Cow<'data, str>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: PlaneType,
    pub team: Team,
    pub pos: Position,
    pub rot: Rotation,
    pub flag: FlagCode,
    pub upgrades: Upgrades,
}
