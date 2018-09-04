use types::{Player, Team};

/// Details about a player that has switched teams.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerReteamPlayer {
	pub id: Player,
	pub team: Team,
}

/// Packet for when players change teams
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerReteam {
	/// List of players that have changed teams.
	pub players: Vec<PlayerReteamPlayer>,
}
