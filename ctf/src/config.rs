use std::time::Duration;

use airmash::Vector2;

pub const BLUE_TEAM: u16 = 1;
pub const RED_TEAM: u16 = 2;

pub const FLAG_RADIUS: f32 = 100.0;

pub const FLAG_HOME_POS: [Vector2<f32>; 2] = [
  // Blue team
  Vector2::new(-9670.0, -1470.0),
  // Red team
  Vector2::new(8600.0, -940.0),
];

pub const TEAM_RESPAWN_POS: [Vector2<f32>; 2] = [
  Vector2::new(-8878.0, -2971.0),
  Vector2::new(7818.0, -2930.0),
];

pub const FLAG_NO_REGRAB_TIME: Duration = Duration::from_secs(5);

/// The base score that a player would get if they were
/// the only ones on the server and they capped. This
/// value will be multiplied by the number of players
/// in the server (up to a max of 10 times).
pub const FLAG_CAP_BOUNTY_BASE: u32 = 100;
/// The base score that a winning player would get
/// if they were the only ones on the server.
pub const GAME_WIN_BOUNTY_BASE: u32 = 100;

pub fn flag_return_pos(team: u16) -> Vector2<f32> {
  FLAG_HOME_POS[(2 - team) as usize]
}
pub fn flag_home_pos(team: u16) -> Vector2<f32> {
  FLAG_HOME_POS[(team - 1) as usize]
}
pub fn team_respawn_pos(team: u16) -> Vector2<f32> {
  TEAM_RESPAWN_POS
    .get((team - 1) as usize)
    .copied()
    .unwrap_or_else(Vector2::zeros)
}

pub fn flag_message_team(team: u16) -> &'static str {
  match team {
    BLUE_TEAM => "blueflag",
    RED_TEAM => "redflag",
    _ => unreachable!(),
  }
}
