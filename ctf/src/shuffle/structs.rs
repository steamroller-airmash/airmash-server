use config::{BLUE_TEAM, RED_TEAM};
use server::{Score, Team};
use specs::Entity;

pub struct PlayerShuffleInfo {
	pub player: Entity,
	pub score: Score,
	pub team: Team,
	pub captures: u32,
	pub kills: u32,
	pub deaths: u32,
}

pub struct TeamChangeEntry {
	pub player: Entity,
	pub new_team: Team,
}

fn switch_team(team: Team) -> Team {
	if team == RED_TEAM {
		BLUE_TEAM
	} else if team == BLUE_TEAM {
		RED_TEAM
	} else {
		team
	}
}

impl From<PlayerShuffleInfo> for TeamChangeEntry {
	fn from(info: PlayerShuffleInfo) -> Self {
		Self {
			player: info.player,
			new_team: switch_team(info.team),
		}
	}
}
