use specs::*;

use crate::component::event::PlayerPowerup;
use crate::component::flag::ForcePlayerUpdate;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

/// Forces a `PlayerUpdate` packet to be sent out when
/// a player is given a powerup.
#[derive(Default)]
pub struct TriggerUpdate;

#[derive(SystemData)]
pub struct TriggerUpdateData<'a> {
  force_update: WriteStorage<'a, ForcePlayerUpdate>,
  entities: Entities<'a>,
}

impl EventHandlerTypeProvider for TriggerUpdate {
  type Event = PlayerPowerup;
}

impl<'a> EventHandler<'a> for TriggerUpdate {
  type SystemData = TriggerUpdateData<'a>;

  fn on_event(&mut self, evt: &PlayerPowerup, data: &mut Self::SystemData) {
    if !data.entities.is_alive(evt.player) {
      return;
    }

    data
      .force_update
      .insert(evt.player, ForcePlayerUpdate)
      .unwrap();
  }
}

impl SystemInfo for TriggerUpdate {
  type Dependencies = super::KnownEventSources;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
