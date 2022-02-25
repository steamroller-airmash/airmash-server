use airmash::AirmashGame;

#[derive(Copy, Clone, Debug, Default)]
pub struct GameScores {
  pub redteam: u8,
  pub blueteam: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct CTFGameStats {
  pub red_players: usize,
  pub blue_players: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct GameActive(pub bool);

pub fn register_all(game: &mut AirmashGame) {
  game.resources.insert(GameScores::default());
  game.resources.insert(GameActive(true));
  game.resources.insert(CTFGameStats::default());
}
