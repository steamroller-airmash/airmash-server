use specs::*;

use crate::component::event::PlayerKilled;
use crate::systems::handlers::game::on_player_hit::AllPlayerHitSystems;
use crate::systems::missile::MissileHit;
use crate::task::death_cooldown;
use crate::types::TaskSpawner;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SetRespawnTimer;

#[derive(SystemData)]
pub struct SetRespawnTimerData<'a> {
  tasks: WriteExpect<'a, TaskSpawner>,
}

impl EventHandlerTypeProvider for SetRespawnTimer {
  type Event = PlayerKilled;
}

impl<'a> EventHandler<'a> for SetRespawnTimer {
  type SystemData = SetRespawnTimerData<'a>;

  fn on_event(&mut self, evt: &PlayerKilled, data: &mut Self::SystemData) {
    let player = evt.player;

    let tdata = data.tasks.task_data();
    data.tasks.launch(death_cooldown(tdata, player));
  }
}

system_info! {
  impl SystemInfo for SetRespawnTimer {
    type Dependencies = (MissileHit, AllPlayerHitSystems);
  }
}
