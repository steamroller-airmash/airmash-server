use crate::component::event::PlayerSpectate;
use crate::protocol::server::PlayerKill;
use crate::types::systemdata::*;
use crate::types::*;
use crate::utils::*;

use crate::SystemInfo;

#[derive(Default)]
pub struct SendKillPacket;

#[derive(SystemData)]
pub struct SendKillPacketData<'a> {
  conns: SendToAll<'a>,
}

impl EventHandlerTypeProvider for SendKillPacket {
  type Event = PlayerSpectate;
}

impl<'a> EventHandler<'a> for SendKillPacket {
  type SystemData = SendKillPacketData<'a>;

  fn on_event(&mut self, evt: &PlayerSpectate, data: &mut Self::SystemData) {
    // If they are already (in spec/dead)
    // we don't need to despawn their plane
    if evt.is_dead || evt.is_spec {
      return;
    }

    // Setting pos to Position::default()
    // indicates to the client that this
    // was a player going into spec.
    let packet = PlayerKill {
      id: evt.player.into(),
      killer: None,
      pos: Position::default(),
    };

    data.conns.send_to_player(evt.player, packet);
  }
}

impl SystemInfo for SendKillPacket {
  type Dependencies = super::KnownEventSources;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
