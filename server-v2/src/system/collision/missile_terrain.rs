use crate::component::{flag::IsMissile, Distance, Mob, Position, Team};
use crate::ecs::prelude::*;
use crate::event::collision::MissileTerrainCollision;
use crate::resource::{
    channel::OnMissileTerrainCollision,
    collision::{HitCircle, Terrain},
};
use crate::system::update_positions;

#[system(deps = update_positions)]
fn missile_terrain_collision<'a>(
    entities: Entities<'a>,
    pos: ReadStorage<'a, Position>,
    mob: ReadStorage<'a, Mob>,
    team: ReadStorage<'a, Team>,
    is_missile: ReadStorage<'a, IsMissile>,

    terrain: Read<'a, Terrain>,

    mut channel: Write<'a, OnMissileTerrainCollision>,
) {
    let iter =
        (&entities, &pos, &team, &mob, &is_missile)
            .join()
            .map(|(ent, &pos, &team, _, ..)| HitCircle {
                pos,
                rad: Distance::new(0.0),
                layer: team.0,
                ent: Some(ent),
            });

    for (missile, terrain) in terrain.collide(iter) {
        channel.single_write(MissileTerrainCollision { missile, terrain });
    }
}
