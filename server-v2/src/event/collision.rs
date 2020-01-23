use crate::ecs::Entity;
use crate::resource::collision::HitCircle;

#[derive(Copy, Clone, Debug)]
pub struct PlayerTerrainCollision {
    pub player: HitCircle,
    pub terrain: HitCircle,
}

impl PlayerTerrainCollision {
    pub fn player(&self) -> Option<Entity> {
        self.player.ent
    }
}
