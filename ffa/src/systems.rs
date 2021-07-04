use server::{
  component::*,
  event::{PlayerJoin, PlayerRespawn},
  resource::collision::LayerSpec,
  resource::collision::Terrain,
  AirmashGame, Vector2,
};

const SPAWN_TOP_RIGHT: Vector2<f32> = Vector2::new(-1325.0, -4330.0);
const SPAWN_SIZE: Vector2<f32> = Vector2::new(3500.0, 2500.0);
const SPAWN_RADIUS: f32 = 100.0;

pub fn select_spawn_position(game: &AirmashGame) -> Vector2<f32> {
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

#[server::handler(priority = server::priority::PRE_LOGIN)]
fn choose_join_position(event: &PlayerJoin, game: &mut AirmashGame) {
  let spawn_pos = select_spawn_position(game);

  if let Some(mut pos) = game.world.get_mut::<Position>(event.player).ok() {
    pos.0 = spawn_pos;
  }
}

#[server::handler(priority = server::priority::HIGH)]
fn choose_respawn_position(event: &PlayerRespawn, game: &mut AirmashGame) {
  let spawn_pos = select_spawn_position(game);

  if let Some(mut pos) = game.world.get_mut::<Position>(event.player).ok() {
    pos.0 = spawn_pos;
  }
}
