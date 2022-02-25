use crate::config;
use airmash::component::*;
use airmash::AirmashGame;
use airmash::{event::PlayerRespawn, Vector2};

#[handler(priority = airmash::priority::MEDIUM)]
fn setup_team_and_pos(event: &PlayerRespawn, game: &mut AirmashGame) {
  let team = match game.world.get::<Team>(event.player) {
    Ok(team) => team.0,
    Err(_) => return,
  };

  let offset = Vector2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
  let respawn = config::team_respawn_pos(team) + 400.0 * offset;

  let _ = game.world.insert_one(event.player, Position(respawn));
}
