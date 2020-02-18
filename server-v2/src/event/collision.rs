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

#[derive(Copy, Clone, Debug)]
pub struct MissileTerrainCollision {
    pub missile: HitCircle,
    pub terrain: HitCircle,
}

impl MissileTerrainCollision {
    pub fn missile(&self) -> Option<Entity> {
        self.missile.ent
    }
}
