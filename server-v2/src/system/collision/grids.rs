use crate::component::{
    flag::{IsMissile, IsPlayer},
    Distance, Position, Team,
};
use crate::ecs::prelude::*;
use crate::resource::collision::{HitCircle, MissileGrid, PlayerGrid};
use crate::sysdata::IsAlive;

use crate::system::{missile::missile_update, update_positions};

#[system(deps = update_positions)]
fn build_player_grid<'a>(
    grid: &mut Write<'a, PlayerGrid>,

    entities: Entities<'a>,
    pos: ReadStorage<'a, Position>,
    is_player: ReadStorage<'a, IsPlayer>,
    team: ReadStorage<'a, Team>,
    is_alive: IsAlive<'a>,
) {
    let circles = (&entities, &pos, &team, is_player.mask() & is_alive.mask())
        .join()
        .map(|(ent, &pos, &team, ..)| HitCircle {
            pos,
            rad: Distance::new(0.0),
            layer: team.0,
            ent: Some(ent),
        })
        .collect();

    grid.rebuild_from(circles);
}

#[system(deps = missile_update)]
fn build_missile_grid<'a>(
    grid: &mut Write<'a, MissileGrid>,

    entities: Entities<'a>,
    pos: ReadStorage<'a, Position>,
    is_missile: ReadStorage<'a, IsMissile>,
    team: ReadStorage<'a, Team>,
) {
    let circles = (&entities, &pos, &team, &is_missile)
        .join()
        .map(|(ent, &pos, &team, ..)| {
            HitCircle {
                pos,
                // TODO: Figure out missile hitcircles
                rad: Distance::new(0.0),
                layer: team.0,
                ent: Some(ent),
            }
        })
        .collect();

    grid.rebuild_from(circles);
}
