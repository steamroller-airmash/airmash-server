use specs::*;

use crate::component::event::{MissileDespawn, MissileDespawnType};
use crate::protocol::server::MobDespawnCoords;
use crate::types::systemdata::*;
use crate::types::Mob;
use crate::utils::{EventHandler, EventHandlerTypeProvider};
use crate::SystemInfo;

/// Add the initial 2s shield when a player joins
/// and send that packet to all visible players.
#[derive(Default)]
pub struct SendMobDespawnCoords;

#[derive(SystemData)]
pub struct SendMobDespawnCoordsData<'a> {
  conns: SendToVisible<'a>,
  mob: ReadStorage<'a, Mob>,
}

impl EventHandlerTypeProvider for SendMobDespawnCoords {
  type Event = MissileDespawn;
}

impl<'a> EventHandler<'a> for SendMobDespawnCoords {
  type SystemData = SendMobDespawnCoordsData<'a>;

  fn on_event(&mut self, evt: &MissileDespawn, data: &mut Self::SystemData) {
    if evt.ty == MissileDespawnType::LifetimeEnded {
      return;
    }

    data.conns.send_to_visible(
      evt.pos,
      MobDespawnCoords {
        id: evt.missile.into(),
        pos: evt.pos,
        ty: *try_get!(evt.missile, data.mob),
      },
    );
  }
}

impl SystemInfo for SendMobDespawnCoords {
  type Dependencies = (super::KnownEventSources);

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
