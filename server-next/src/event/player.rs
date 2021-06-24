use hecs::Entity;

#[derive(Clone, Copy, Debug)]
pub struct PlayerJoin {
  pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerLeave {
  pub player: Entity,
}
