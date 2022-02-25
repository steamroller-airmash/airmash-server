use specs::*;

use crate::component::event::PlayerRespawn as EvtPlayerRespawn;
use crate::types::systemdata::SendToVisible;
use crate::types::*;
use crate::SystemInfo;

use crate::protocol::server::PlayerRespawn;
use crate::protocol::Upgrades as ProtocolUpgrades;

use crate::utils::{EventHandler, EventHandlerTypeProvider};

use crate::systems::handlers::command::AllCommandHandlers;
use crate::systems::handlers::game::on_join::AllJoinHandlers;
use crate::systems::handlers::game::on_player_respawn::SetTraits;

/// Send a [`PlayerRespawn`] packet to
/// all visible players if the target
/// player is not currently spectating.
#[derive(Default)]
pub struct SendPlayerRespawn;

#[derive(SystemData)]
pub struct SendPlayerRespawnData<'a> {
  entities: Entities<'a>,
  conns: SendToVisible<'a>,

  pos: ReadStorage<'a, Position>,
  rot: ReadStorage<'a, Rotation>,
}

impl EventHandlerTypeProvider for SendPlayerRespawn {
  type Event = EvtPlayerRespawn;
}

impl<'a> EventHandler<'a> for SendPlayerRespawn {
  type SystemData = SendPlayerRespawnData<'a>;

  fn on_event(&mut self, evt: &EvtPlayerRespawn, data: &mut Self::SystemData) {
    if !data.entities.is_alive(evt.player) {
      return;
    }

    let player = evt.player;
    let pos = *try_get!(player, data.pos);
    let rot = *try_get!(player, data.rot);
    let packet = PlayerRespawn {
      id: player.into(),
      pos: pos,
      rot: rot,
      upgrades: ProtocolUpgrades::default(),
    };

    // FIXME: Bake setting traits into a respawn task
    //        so that there's a 1-frame gap between
    //        setting the position and respawning. This
    //        would ensure that the collision mask is
    //        properly updated before.
    //
    // Alternative: Allow reinserting players into the
    //        collision grid mid-frame. This one might
    //        actually be better.
    data.conns.send_to_others_visible(pos, player, packet);

    data.conns.send_to_player(player, packet);
  }
}

impl SystemInfo for SendPlayerRespawn {
  type Dependencies = (AllJoinHandlers, SetTraits, AllCommandHandlers);

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
