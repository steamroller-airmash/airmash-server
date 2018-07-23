use server::protocol::GameType;
use server::*;

use rand;
use specs::Entity;

use std::cmp::Ordering;

pub const RED_TEAM: Team = Team(2);
pub const BLUE_TEAM: Team = Team(1);

lazy_static! {
	static ref BLUE_TEAM_RESPAWN: Position =
		Position::new(Distance::new(-8878.0), Distance::new(-2971.0));
	static ref RED_TEAM_RESPAWN: Position =
		Position::new(Distance::new(7818.0), Distance::new(-2930.0));
}

#[derive(Default, Debug)]
pub struct CTFGameMode {
	pub redteam: u16,
	pub blueteam: u16,
}

impl CTFGameMode {
	pub fn new() -> Self {
		Self::default()
	}
}

impl GameMode for CTFGameMode {
	fn assign_team(&mut self, _: Entity) -> Team {
		info!("Teams: {} blue, {} red", self.blueteam, self.redteam);
		match self.redteam.cmp(&self.blueteam) {
			Ordering::Less => {
				info!("Added to red team");
				self.redteam += 1;
				RED_TEAM
			}
			Ordering::Greater => {
				self.blueteam += 1;
				info!("Added to blue team");
				BLUE_TEAM
			}
			Ordering::Equal => {
				let team: bool = rand::random();

				if team {
					self.redteam += 1;
					info!("Added to red team");
					RED_TEAM
				} else {
					self.blueteam += 1;
					info!("Added to blue team");
					BLUE_TEAM
				}
			}
		}
	}

	fn spawn_pos(&mut self, _: Entity, team: Team) -> Position {
		if team == BLUE_TEAM {
			*BLUE_TEAM_RESPAWN
		} else if team == RED_TEAM {
			*RED_TEAM_RESPAWN
		} else {
			// No need for this yet
			unimplemented!();
		}
	}

	fn gametype(&self) -> GameType {
		GameType::CTF
	}

	fn room(&self) -> String {
		"matrix".to_owned()
	}
}
