use crate::component::{flag::IsPlayer, Plane, Position, Rotation, Team};
use crate::data::PLANE_HIT_CIRCLES;
use crate::ecs::prelude::*;
use crate::event::collision::PlayerTerrainCollision;
use crate::resource::channel::OnPlayerTerrainCollision;
use crate::resource::collision::*;
use crate::system::update_positions;

#[system(deps = update_positions)]
fn player_terrain_collision<'a>(
    entities: Entities<'a>,
    pos: ReadStorage<'a, Position>,
    rot: ReadStorage<'a, Rotation>,
    team: ReadStorage<'a, Team>,
    plane: ReadStorage<'a, Plane>,
    is_player: ReadStorage<'a, IsPlayer>,

    terrain: Read<'a, Terrain>,

    mut channel: Write<'a, OnPlayerTerrainCollision>,
) {
    let iter = (&entities, &pos, &plane, &rot, &team, &is_player)
        .join()
        .flat_map(|(ent, &pos, &plane, &rot, &team, ..)| {
            PLANE_HIT_CIRCLES[plane].iter().copied().map(move |hc| {
                let offset = hc.offset.rotate(rot);

                HitCircle {
                    pos: pos + offset,
                    rad: hc.radius,
                    layer: team.0,
                    ent: Some(ent),
                }
            })
        });

    for (player, terrain) in terrain.collide(iter) {
        channel.single_write(PlayerTerrainCollision { player, terrain });
    }
}
