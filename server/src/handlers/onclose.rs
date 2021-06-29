use crate::types::event::ConnectionClose;
use crate::types::*;
use specs::*;

use crate::component::channel::OnPlayerLeave;
use crate::component::counter::PlayersGame;
use crate::component::event::PlayerLeave as EvtPlayerLeave;
use crate::dispatch::SystemInfo;
use crate::handlers::OnOpenHandler;

use crate::utils::*;

#[derive(Default)]
pub struct OnCloseHandler;

impl EventHandlerTypeProvider for OnCloseHandler {
  type Event = ConnectionClose;
}

impl<'a> EventHandler<'a> for OnCloseHandler {
  type SystemData = (
    Entities<'a>,
    Write<'a, Connections>,
    Write<'a, PlayersGame>,
    Write<'a, OnPlayerLeave>,
  );

  fn on_event(
    &mut self,
    evt: &ConnectionClose,
    (entities, connections, players, onleave): &mut Self::SystemData,
  ) {
    let (player, ty) = {
      let conn = match connections.conns.get(&evt.conn) {
        Some(c) => c,
        None => {
          // This can sometimes happen legitimately if a disconnect occurrs.
          return;
        }
      };

      (conn.player, conn.ty)
    };

    if ty == ConnectionType::Primary {
      if let Some(ent) = player {
        connections.remove_player(ent);
        players.0 -= 1;

        onleave.single_write(EvtPlayerLeave(ent));
        // Delete player entity
        entities.delete(ent).unwrap();

        // Log
        info!("Player {:?} left", ent);
      } else {
        connections.remove(evt.conn);
      }
    } else {
      connections.remove(evt.conn);
    }
  }
}

impl SystemInfo for OnCloseHandler {
  type Dependencies = OnOpenHandler;

  fn new() -> Self {
    Self::default()
  }

  fn name() -> &'static str {
    concat!(module_path!(), line!())
  }
}
