use specs::*;

use crate::server::component::event::PlayerLeave;
use crate::server::systems::handlers::packet::OnCloseHandler;
use crate::server::utils::*;
use crate::server::*;

use crate::CTFGameMode;
use crate::BLUE_TEAM;
use crate::RED_TEAM;

#[derive(Default)]
pub struct UpdateGameModeOnPlayerLeave;

#[derive(SystemData)]
pub struct UpdateGameModeOnPlayerLeaveData<'a> {
  gamemode: GameModeWriter<'a, CTFGameMode>,
  teams: ReadStorage<'a, Team>,
}

impl EventHandlerTypeProvider for UpdateGameModeOnPlayerLeave {
  type Event = PlayerLeave;
}

impl<'a> EventHandler<'a> for UpdateGameModeOnPlayerLeave {
  type SystemData = UpdateGameModeOnPlayerLeaveData<'a>;

  fn on_event(&mut self, &PlayerLeave(ent): &PlayerLeave, data: &mut Self::SystemData) {
    let team = try_get!(ent, data.teams);

    if *team == RED_TEAM {
      data.gamemode.redteam -= std::cmp::min(data.gamemode.redteam, 1);
    } else if *team == BLUE_TEAM {
      data.gamemode.blueteam -= std::cmp::min(data.gamemode.blueteam, 1);
    } else {
      unimplemented!();
    }
  }
}

impl SystemInfo for UpdateGameModeOnPlayerLeave {
  type Dependencies = OnCloseHandler;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
