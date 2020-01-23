use crate::component::{flag::IsPlayer, Distance, Position, Team};
use crate::ecs::prelude::*;
use crate::resource::collision::{HitCircle, PlayerGrid};
use crate::sysdata::IsAlive;

use crate::system::update_positions;

#[system(deps = update_positions)]
fn build_player_grid<'a>(
    grid: &mut Write<'a, PlayerGrid>,

    entities: Entities<'a>,
    pos: &ReadStorage<'a, Position>,
    is_player: &ReadStorage<'a, IsPlayer>,
    team: &ReadStorage<'a, Team>,
    is_alive: IsAlive<'a>,
) {
    let circles = (&entities, pos, team, is_player.mask() & is_alive.mask())
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
