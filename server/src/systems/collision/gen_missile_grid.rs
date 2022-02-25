use specs::*;

use crate::types::collision::*;
use crate::types::*;

use crate::component::collision::MissileGrid;
use crate::component::flag::IsMissile;

#[derive(Default)]
pub struct GenMissileGrid;

#[derive(SystemData)]
pub struct GenMissileGridData<'a> {
  grid: Write<'a, MissileGrid>,

  entities: Entities<'a>,
  pos: ReadStorage<'a, Position>,
  team: ReadStorage<'a, Team>,
  is_missile: ReadStorage<'a, IsMissile>,
}

impl<'a> System<'a> for GenMissileGrid {
  type SystemData = GenMissileGridData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    let it = (
      &*data.entities,
      &data.pos,
      &data.team,
      data.is_missile.mask(),
    )
      .join()
      .map(|(ent, pos, team, ..)| HitCircle {
        pos: *pos,
        rad: Distance::new(0.0),
        ent: ent,
        layer: team.0,
      });

    data.grid.0.rebuild_from(it);
  }
}

system_info! {
  impl SystemInfo for GenMissileGrid {
    type Dependencies = (
      crate::systems::PositionUpdate,
      crate::systems::handlers::game::on_missile_despawn::KnownEventSources
    );
  }
}
