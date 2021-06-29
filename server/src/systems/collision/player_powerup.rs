use hashbrown::HashSet;
use specs::prelude::*;

use crate::Mob;

use crate::types::collision::*;
use crate::types::*;

use crate::component::channel::*;
use crate::component::collision::PlaneGrid;
use crate::component::event::PlayerPowerupCollision;
use crate::component::flag::*;

#[derive(Default)]
pub struct PlayerPowerupCollisionSystem;

#[derive(SystemData)]
pub struct PlayerPowerupCollisionSystemData<'a> {
  channel: Write<'a, OnPlayerPowerupCollision>,
  ent: Entities<'a>,
  grid: Read<'a, PlaneGrid>,

  pos: ReadStorage<'a, Position>,
  mob: ReadStorage<'a, Mob>,
  is_powerup: ReadStorage<'a, IsPowerup>,
}

impl<'a> System<'a> for PlayerPowerupCollisionSystem {
  type SystemData = PlayerPowerupCollisionSystemData<'a>;

  fn run(&mut self, data: Self::SystemData) {
    let mut channel = data.channel;
    let grid = &data.grid.0;

    let collisions = (&*data.ent, &data.pos, &data.mob, data.is_powerup.mask())
      .join()
      .map(|(ent, pos, mob, ..)| {
        let it = COLLIDERS[mob].iter().map(|(offset, rad)| HitCircle {
          pos: *pos + *offset,
          rad: *rad,
          layer: 0,
          ent: ent,
        });

        grid.collide(it)
      })
      .flatten()
      .map(PlayerPowerupCollision)
      .collect::<HashSet<_>>();

    channel.iter_write(collisions.into_iter());
  }
}

use super::GenPlaneGrid;
use crate::systems::PositionUpdate;

system_info! {
  impl SystemInfo for PlayerPowerupCollisionSystem {
    type Dependencies = (PositionUpdate, GenPlaneGrid);
  }
}
