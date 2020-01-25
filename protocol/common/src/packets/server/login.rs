use crate::enums::{FlagCode, GameType, PlaneType, PlayerStatus};
use crate::types::{Level, Player, Position, Rotation, Team, Upgrades};

use std::borrow::Cow;

/// Initial data passed in for a
/// player when the server starts.
///
/// This is an element of the `players`
/// array within the [`Login`] packet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LoginPlayer<'data> {
    pub id: Player,
    pub status: PlayerStatus,
    pub level: Level,
    pub name: Cow<'data, str>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: PlaneType,
    pub team: Team,
    pub pos: Position,
    pub rot: Rotation,
    pub flag: FlagCode,
    pub upgrades: Upgrades,
}

/// Initial Login packet sent to the server
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Login<'data> {
    pub success: bool,
    pub id: Player,
    pub team: Team,
    pub clock: u32,
    pub token: Cow<'data, str>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: GameType,
    pub room: Cow<'data, str>,
    pub players: Vec<LoginPlayer<'data>>,
}
