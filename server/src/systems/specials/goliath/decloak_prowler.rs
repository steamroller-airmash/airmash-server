use crate::types::systemdata::*;
use crate::types::*;
use specs::*;

use crate::SystemInfo;

use crate::component::event::PlayerRepel;
use crate::component::flag::IsPlayer;
use crate::component::time::{LastStealthTime, ThisFrame};
use crate::systems::specials::config::*;

use crate::protocol::server::EventStealth;
use crate::utils::{EventHandler, EventHandlerTypeProvider};

/// Send [`EventRepel`][0] when a goliath uses it's special.
///
/// This system also position, speed, velocity,
/// team and owner for players and mobs that
/// are caught in the deflection.
///
/// [0]: airmash_protocol::server::EventRepel
#[derive(Default)]
pub struct DecloakProwler;

#[derive(SystemData)]
pub struct DecloakProwlerData<'a> {
  entities: Entities<'a>,
  this_frame: Read<'a, ThisFrame>,
  conns: SendToAll<'a>,

  pos: ReadStorage<'a, Position>,
  team: WriteStorage<'a, Team>,
  keystate: WriteStorage<'a, KeyState>,
  last_stealth: WriteStorage<'a, LastStealthTime>,
  is_alive: IsAlive<'a>,
  is_player: ReadStorage<'a, IsPlayer>,

  energy: ReadStorage<'a, Energy>,
  energy_regen: ReadStorage<'a, EnergyRegen>,
}

impl EventHandlerTypeProvider for DecloakProwler {
  type Event = PlayerRepel;
}

impl<'a> EventHandler<'a> for DecloakProwler {
  type SystemData = DecloakProwlerData<'a>;

  fn on_event(&mut self, evt: &PlayerRepel, data: &mut Self::SystemData) {
    let pos = *try_get!(evt.player, data.pos);
    let team = *try_get!(evt.player, data.team);
    let player_r2 = *GOLIATH_SPECIAL_RADIUS_PLAYER * *GOLIATH_SPECIAL_RADIUS_PLAYER;

    let hit_players = (
      &*data.entities,
      &data.pos,
      &data.team,
      data.is_player.mask(),
      data.is_alive.mask(),
    )
      .join()
      .filter(|(ent, ..)| *ent != evt.player)
      .filter(|(_, _, player_team, ..)| **player_team != team)
      .filter_map(|(ent, player_pos, ..)| {
        let dist2 = (*player_pos - pos).length2();

        if dist2 < player_r2 {
          Some(ent)
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    for player in hit_players {
      let keystate = try_get!(player, mut data.keystate);

      keystate.stealthed = false;
      keystate.special = false;

      data
        .last_stealth
        .insert(player, LastStealthTime(data.this_frame.0))
        .unwrap();

      data.conns.send_to_player(
        player,
        EventStealth {
          id: player.into(),
          energy: *try_get!(player, data.energy),
          energy_regen: *try_get!(player, data.energy_regen),
          state: false,
        },
      );
    }
  }
}

impl SystemInfo for DecloakProwler {
  type Dependencies = super::GoliathRepel;

  fn name() -> &'static str {
    concat!(module_path!(), "::", line!())
  }

  fn new() -> Self {
    Self::default()
  }
}
