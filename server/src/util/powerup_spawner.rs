use std::time::{Duration, Instant};

use crate::event::Frame;
use crate::protocol::{MobType, Position};
use crate::{Entity, EventHandler};

#[derive(Copy, Clone)]
enum SpawnerState {
  Spawned(Entity),
  Unspawned(Instant),
}

pub struct PeriodicPowerupSpawner {
  state: SpawnerState,
  interval: Duration,
  mob: MobType,
  pos: Position,
}

impl PeriodicPowerupSpawner {
  pub fn new(mob: MobType, pos: Position, interval: Duration) -> Self {
    Self {
      mob,
      pos,
      interval,
      state: SpawnerState::Unspawned(Instant::now()),
    }
  }

  pub fn inferno(pos: Position, interval: Duration) -> Self {
    Self::new(MobType::Inferno, pos, interval)
  }

  pub fn shield(pos: Position, interval: Duration) -> Self {
    Self::new(MobType::Shield, pos, interval)
  }
}

impl EventHandler<Frame> for PeriodicPowerupSpawner {
  fn on_event(&mut self, _: &Frame, game: &mut crate::AirmashGame) {
    let frame = game.this_frame();

    match self.state {
      SpawnerState::Spawned(entity) => {
        if !game.world.contains(entity) {
          self.state = SpawnerState::Unspawned(frame + self.interval);
        }
      }
      SpawnerState::Unspawned(next) => {
        if frame > next {
          let entity = game.spawn_mob(self.mob, self.pos, Duration::from_secs(60));
          self.state = SpawnerState::Spawned(entity);
        }
      }
    }
  }
}
