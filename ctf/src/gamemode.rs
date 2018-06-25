use server::*;

use rand;
use specs::Entity;

use std::cmp::Ordering;

const RED_TEAM: Team = Team(2);
const BLUE_TEAM: Team = Team(1);

lazy_static! {
    static ref RED_TEAM_RESPAWN: Position = Position::default();
    static ref BLUE_TEAM_RESPAWN: Position = Position::default();
}

#[derive(Default, Debug)]
pub struct CTFGameMode {
    redteam: u16,
    blueteam: u16,
}

impl CTFGameMode {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GameMode for CTFGameMode {
    fn assign_team(&mut self, _: Entity) -> Team {
        match self.redteam.cmp(&self.blueteam) {
            Ordering::Less => {
                self.redteam += 1;
                RED_TEAM
            }
            Ordering::Greater => {
                self.blueteam += 1;
                BLUE_TEAM
            }
            Ordering::Equal => {
                let team: bool = rand::random();

                if team {
                    self.redteam += 1;
                    RED_TEAM
                } else {
                    self.blueteam += 1;
                    BLUE_TEAM
                }
            }
        }
    }

    fn respawn_pos(&mut self, _: Entity, team: Team) -> Position {
        if team == BLUE_TEAM {
            *BLUE_TEAM_RESPAWN
        } else if team == RED_TEAM {
            *RED_TEAM_RESPAWN
        } else {
            // No need for this yet
            unimplemented!();
        }
    }
}
