use specs::*;

use crate::component::event::*;
use crate::types::*;
use crate::SystemInfo;

use crate::systems::handlers::command::AllCommandHandlers;
use crate::systems::handlers::game::on_join::AllJoinHandlers;

use crate::utils::{EventHandler, EventHandlerTypeProvider};

/// Reset the keystate of a player when they
/// respawn.
#[derive(Default)]
pub struct ResetKeyState;

#[derive(SystemData)]
pub struct ResetKeyStateData<'a> {
  entities: Entities<'a>,
  keystate: WriteStorage<'a, KeyState>,
}

impl EventHandlerTypeProvider for ResetKeyState {
  type Event = PlayerRespawn;
}

impl<'a> EventHandler<'a> for ResetKeyState {
  type SystemData = ResetKeyStateData<'a>;

  fn on_event(&mut self, evt: &PlayerRespawn, data: &mut Self::SystemData) {
    if !data.entities.is_alive(evt.player) {
      return;
    }

    data
      .keystate
      .insert(evt.player, KeyState::default())
      .unwrap();
  }
}

impl SystemInfo for ResetKeyState {
  type Dependencies = (AllJoinHandlers, AllCommandHandlers);

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
