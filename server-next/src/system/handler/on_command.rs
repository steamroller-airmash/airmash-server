use std::convert::TryFrom;
use std::time::{Duration, Instant};

use airmash_protocol::PlaneType;
use bstr::BString;
use bstr::ByteSlice;

use crate::component::*;
use crate::event::{PacketEvent, PlayerChangePlane, PlayerRespawn};
use crate::protocol::client::Command;
use crate::protocol::{server as s, ErrorType};
use crate::resource::{GameConfig, ThisFrame};
use crate::AirmashWorld;

#[handler]
fn on_respawn_command(event: &PacketEvent<Command>, game: &mut AirmashWorld) {
  fn respawn_allowed(
    alive: bool,
    respawn_allowed: bool,
    health: f32,
    last_action: Instant,
    this_frame: Instant,
  ) -> bool {
    // Note to my future self and maintainers:
    //  Originally this code was written as one big boolean expression. This was
    //  unclear and caused some bugs so now it's been rewritten in this fashion.
    //  This is a lot clearer and I'd prefer if it stayed that way.

    // Another note:
    //  This function explicitly doesn't check the velocity of a player since
    //  respawning while moving has always been possible in airmash. Whether
    //  this is a bug in the  original server is debatable but I'd like to stay
    //  true to the original server if possible.

    // A player may not respawn during the 2s cooldown period after dying (this
    // is represented by the RespawnAllowed flag). This takes priority over
    // whether a player is spectating.
    if !respawn_allowed {
      trace!("respawn denied - 2s cooldown after death");
      return false;
    }

    // If the player is spectating then they may respawn at any time. Note that
    // respawn_allowed will prevent respawning during the first 2 seconds after
    // going into spec.
    //
    // A player that is spectating is dead but allowed to respawn.
    if !alive {
      trace!("respawn allowed - is spectating");
      return true;
    }

    // Players that don't have full health may not respawn
    if health < 1.0 {
      trace!("respawn denied - poor health");
      return false;
    }

    // Players that have performed an action within the last 2s may not respawn.
    if (this_frame - last_action) < Duration::from_secs(2) {
      trace!("respawn denied - pressed key too recently");
      return false;
    }

    true
  }

  fn parse_plane(s: &BString) -> Result<PlaneType, ()> {
    let s = s.to_str().map_err(|_| ())?;
    let num: u32 = s.parse().map_err(|_| ())?;
    PlaneType::try_from(num).map_err(|_| ())
  }

  if !game.resources.read::<GameConfig>().allow_respawn {
    return;
  }

  if event.packet.com != "respawn" {
    return;
  }

  let newplane = match parse_plane(&event.packet.data) {
    Ok(newplane) => newplane,
    Err(_) => return,
  };

  let this_frame = game.resources.read::<ThisFrame>().0;

  let mut query = match game.world.query_one::<(
    &RespawnAllowed,
    &mut IsAlive,
    &Health,
    &LastActionTime,
    &mut PlaneType,
  )>(event.entity)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  let (&allowed, alive, &health, &last_action, plane) = match query.get() {
    Some(query) => query,
    None => return,
  };

  if !respawn_allowed(alive.0, allowed.0, health.0, last_action.0, this_frame) {
    game.send_to(
      event.entity,
      s::Error {
        error: ErrorType::IdleRequiredBeforeRespawn,
      },
    );
    return;
  }

  let oldplane = std::mem::replace(plane, newplane);
  let prev_alive = std::mem::replace(&mut alive.0, true);

  drop(query);

  game.dispatch(PlayerRespawn {
    player: event.entity,
    alive: prev_alive,
  });

  if oldplane != newplane {
    game.dispatch(PlayerChangePlane {
      player: event.entity,
      old_plane: oldplane,
    });
  }
}
