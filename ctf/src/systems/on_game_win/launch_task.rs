use specs::prelude::*;

use crate::component::GameWinEvent;
use crate::server::types::TaskSpawner;
use crate::server::utils::{EventHandler, EventHandlerTypeProvider};
use crate::systems::on_flag::CheckWin;
use crate::tasks::new_game;

#[derive(SystemData)]
pub struct LaunchTaskData<'a> {
  task: WriteExpect<'a, TaskSpawner>,
}

#[derive(Default)]
pub struct LaunchTask;

impl EventHandlerTypeProvider for LaunchTask {
  type Event = GameWinEvent;
}

impl<'a> EventHandler<'a> for LaunchTask {
  type SystemData = LaunchTaskData<'a>;

  fn on_event(&mut self, evt: &GameWinEvent, data: &mut Self::SystemData) {
    let tdata = data.task.task_data();
    data.task.launch(new_game(tdata, evt.winning_team));
  }
}

system_info! {
  impl SystemInfo for LaunchTask {
    type Dependencies = CheckWin;
  }
}
