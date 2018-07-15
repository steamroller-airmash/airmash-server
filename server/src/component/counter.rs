use specs::*;
use types::Score;

#[derive(Clone, Debug, Copy, Component, Default)]
pub struct PlayersGame(pub u32);

#[derive(Clone, Debug, Copy, Component, Default)]
pub struct TotalKills(pub u32);

#[derive(Clone, Debug, Copy, Component, Default)]
pub struct TotalDeaths(pub u32);

#[derive(Clone, Debug, Copy, Component, Default)]
pub struct Earnings(pub Score);

/// Player ping in ms
#[derive(Clone, Debug, Copy, Component, Default)]
pub struct PlayerPing(pub u32);
