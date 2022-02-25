use crate::types::*;
use specs::*;

use crate::SystemInfo;

use crate::component::event::*;
use crate::protocol::server::PlayerLevel;
use crate::protocol::PlayerLevelType;
use crate::types::systemdata::SendToAll;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

#[derive(Default)]
pub struct SendPlayerLevel;

#[derive(SystemData)]
pub struct SendPlayerLevelData<'a> {
  conns: SendToAll<'a>,

  level: ReadStorage<'a, Level>,
}

impl EventHandlerTypeProvider for SendPlayerLevel {
  type Event = PlayerJoin;
}

impl<'a> EventHandler<'a> for SendPlayerLevel {
  type SystemData = SendPlayerLevelData<'a>;

  fn on_event(&mut self, evt: &PlayerJoin, data: &mut Self::SystemData) {
    let packet = PlayerLevel {
      id: evt.id.into(),
      ty: PlayerLevelType::Login,
      level: *try_get!(evt.id, data.level),
    };

    data.conns.send_to_others(evt.id, packet);
  }
}

impl SystemInfo for SendPlayerLevel {
  type Dependencies = (
    // super::InitTraits,
    super::SendLogin,
    super::InitConnection,
    super::SendPlayerNew,
  );

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
