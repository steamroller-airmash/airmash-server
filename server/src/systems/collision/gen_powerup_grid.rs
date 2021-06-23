use specs::*;

use crate::consts::config::POWERUP_RADIUS;
use crate::types::collision::*;
use crate::types::*;

use crate::component::collision::PowerupGrid;
use crate::component::flag::IsPowerup;

#[derive(Default)]
pub struct GenPowerupGrid;

#[derive(SystemData)]
pub struct GenPowerupGridData<'a> {
  grid: Write<'a, PowerupGrid>,

  entities: Entities<'a>,
  pos: ReadStorage<'a, Position>,
  is_powerup: ReadStorage<'a, IsPowerup>,
}

impl<'a> System<'a> for GenPowerupGrid {
  type SystemData = GenPowerupGridData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    let it = (&*data.entities, &data.pos, data.is_powerup.mask())
      .join()
      .map(|(ent, pos, ..)| HitCircle {
        pos: *pos,
        rad: Distance::new(POWERUP_RADIUS.inner()),
        ent: ent,
        layer: 0,
      });

    data.grid.rebuild_from(it);
  }
}

system_info! {
  impl SystemInfo for GenPowerupGrid {
    type Dependencies = crate::systems::PositionUpdate;
  }
}
