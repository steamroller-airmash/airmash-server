use specs::*;

use crate::component::event::WhisperEvent;
use crate::component::flag::*;
use crate::protocol::server::{ChatWhisper, Error};
use crate::protocol::ErrorType;
use crate::types::systemdata::SendToPlayer;

use crate::utils::*;

use crate::component::flag::IsPlayer;

#[derive(Default)]
pub struct WhisperHandler;

#[derive(SystemData)]
pub struct WhisperHandlerData<'a> {
  conns: SendToPlayer<'a>,

  throttled: ReadStorage<'a, IsChatThrottled>,
  muted: ReadStorage<'a, IsChatMuted>,

  entities: Entities<'a>,
  is_player: ReadStorage<'a, IsPlayer>,
}

impl EventHandlerTypeProvider for WhisperHandler {
  type Event = WhisperEvent;
}

impl<'a> EventHandler<'a> for WhisperHandler {
  type SystemData = WhisperHandlerData<'a>;

  fn on_event(&mut self, evt: &WhisperEvent, data: &mut Self::SystemData) {
    info!("{:?}", evt);
    let player = match data.conns.associated_player(evt.0) {
      Some(player) => player,
      None => return,
    };

    if data.muted.get(player).is_some() {
      return;
    }
    if data.throttled.get(player).is_some() {
      data.conns.send_to(
        evt.0,
        Error {
          error: ErrorType::ChatThrottled,
        },
      );
      return;
    }

    let to = data.entities.entity(evt.1.id.0 as u32);

    if !data.entities.is_alive(to) {
      // The player doesn't exist
      return;
    }
    if data.is_player.get(to).is_none() {
      // Entity is not a player
      return;
    }

    let chat = ChatWhisper {
      from: player.into(),
      to: to.into(),
      text: evt.1.text.clone(),
    };

    let packet = chat.into();

    data.conns.send_to_ref(evt.0, &packet);

    data.conns.send_to_player(to, packet);
  }
}

system_info! {
  impl SystemInfo for WhisperHandler {
    type Dependencies = crate::handlers::OnCloseHandler;
  }
}
