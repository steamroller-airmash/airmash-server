use crate::types::*;
use specs::*;

use crate::component::channel::OnPlayerDespawn;
use crate::component::event::{
  PlayerDespawn, PlayerDespawnType, PlayerRespawn, PlayerRespawnPrevStatus,
};

use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

/// Create a despawn event when a player respawns.
/// If that player was alive at the time of respawn.
#[derive(Default)]
pub struct CreateDespawnEvent;

#[derive(SystemData)]
pub struct CreateDespawnEventData<'a> {
  channel: Write<'a, OnPlayerDespawn>,
  pos: ReadStorage<'a, Position>,
}

impl EventHandlerTypeProvider for CreateDespawnEvent {
  type Event = PlayerRespawn;
}

impl<'a> EventHandler<'a> for CreateDespawnEvent {
  type SystemData = CreateDespawnEventData<'a>;

  fn on_event(&mut self, evt: &PlayerRespawn, data: &mut Self::SystemData) {
    // If the player wasn't alive, then they didn't despawn
    if evt.prev_status != PlayerRespawnPrevStatus::Alive {
      return;
    }

    let &pos = try_get!(evt.player, data.pos);

    data.channel.single_write(PlayerDespawn {
      ty: PlayerDespawnType::Respawn,
      player: evt.player,
      pos,
    })
  }
}

impl SystemInfo for CreateDespawnEvent {
  type Dependencies = super::KnownEventSources;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
