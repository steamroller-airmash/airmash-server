use airmash::component::*;
use airmash::event::{PlayerJoin, PlayerRespawn};
use airmash::resource::collision::{LayerSpec, Terrain};
use airmash::{AirmashGame, Vector2};

const SPAWN_TOP_RIGHT: Vector2 = Vector2::new(-1325.0, -4330.0);
const SPAWN_SIZE: Vector2 = Vector2::new(3500.0, 2500.0);
const SPAWN_RADIUS: f32 = 100.0;

pub fn select_spawn_position(game: &AirmashGame) -> Vector2 {
  let terrain = game.resources.read::<Terrain>();

  loop {
    let pos = SPAWN_TOP_RIGHT
      + Vector2::new(
        rand::random::<f32>() * SPAWN_SIZE.x,
        rand::random::<f32>() * SPAWN_SIZE.y,
      );

    if !terrain.contains(pos, SPAWN_RADIUS, LayerSpec::None) {
      break pos;
    }
  }
}

#[airmash::handler(priority = airmash::priority::PRE_LOGIN)]
fn choose_join_position(event: &PlayerJoin, game: &mut AirmashGame) {
  let spawn_pos = select_spawn_position(game);

  if let Ok(mut pos) = game.world.get_mut::<Position>(event.player) {
    pos.0 = spawn_pos;
  }
}

#[airmash::handler(priority = airmash::priority::HIGH)]
fn choose_respawn_position(event: &PlayerRespawn, game: &mut AirmashGame) {
  let spawn_pos = select_spawn_position(game);

  if let Ok(mut pos) = game.world.get_mut::<Position>(event.player) {
    pos.0 = spawn_pos;
  }
}
