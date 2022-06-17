use std::convert::TryFrom;
use std::str::FromStr;
use std::time::{Duration, Instant};

use bstr::{BString, ByteSlice};

use crate::component::*;
use crate::config::PlanePrototype;
use crate::event::{PacketEvent, PlayerChangePlane, PlayerRespawn, PlayerSpectate};
use crate::protocol::client::Command;
use crate::protocol::server::PlayerFlag;
use crate::protocol::{server as s, ErrorType, PlaneType, UpgradeType};
use crate::resource::{GameConfig, ThisFrame};
use crate::util::spectate::*;
use crate::AirmashGame;

#[handler]
fn on_respawn_command(event: &PacketEvent<Command>, game: &mut AirmashGame) {
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
  let gconfig = game.resources.read::<GameConfig>().inner;

  let mut query = match game.world.query_one::<(
    &RespawnAllowed,
    &mut IsAlive,
    &Health,
    &LastActionTime,
    &mut PlaneType,
    &mut &'static PlanePrototype,
  )>(event.entity)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  let (&allowed, alive, &health, &last_action, plane, proto) = match query.get() {
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

  let pname = match plane {
    PlaneType::Predator => "predator",
    PlaneType::Tornado => "tornado",
    PlaneType::Goliath => "goliath",
    PlaneType::Prowler => "prowler",
    PlaneType::Mohawk => "mohawk",
  };
  *proto = match gconfig.planes.get(pname) {
    Some(proto) => proto,
    None => {
      game.send_to(
        event.entity,
        s::ServerMessage {
          ty: crate::protocol::ServerMessageType::Banner,
          duration: 5000,
          text: format!("{:?} is not available on this server", plane).into(),
        },
      );
      return;
    }
  };

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

#[handler]
fn on_flag_command(event: &PacketEvent<Command>, game: &mut AirmashGame) {
  if event.packet.com != "flag" {
    return;
  }

  let (flag, _) = match game
    .world
    .query_one_mut::<(&mut FlagCode, &IsPlayer)>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let newflag = FlagCode::from_str(event.packet.data.to_str().unwrap_or("UN"))
    .unwrap_or(FlagCode::UnitedNations);
  *flag = newflag;

  game.send_to_all(PlayerFlag {
    id: event.entity.id() as _,
    flag: newflag,
  });
}

#[handler]
fn on_spectate_command(event: &PacketEvent<Command>, game: &mut AirmashGame) {
  fn can_spectate(
    is_spec: bool,
    is_alive: bool,
    health: f32,
    last_action: Instant,
    this_frame: Instant,
  ) -> bool {
    // If the player is spectating then they may change who they are spectating at
    // any time. Similarly, if they are dead then they may spectate at any time.
    if is_spec || !is_alive {
      return true;
    }

    // Spectating requires full health.
    if health < 1.0 {
      return false;
    }

    // A player must have been idle for 2 seconds to spectate.
    if this_frame - last_action < Duration::from_secs(2) {
      return false;
    }

    true
  }

  fn parse_spectate_data(s: &str) -> Result<SpectateTarget, ()> {
    let arg: i32 = s.parse().map_err(|_| ())?;

    if arg < -3 || arg > u16::MAX as _ {
      return Err(());
    }

    Ok(match arg {
      -1 => SpectateTarget::Next,
      -2 => SpectateTarget::Prev,
      -3 => SpectateTarget::Force,
      x => SpectateTarget::Target(x as u16),
    })
  }

  if event.packet.com != "spectate" {
    return;
  }

  let tgt = match parse_spectate_data(event.packet.data.to_str().unwrap_or("")) {
    Ok(tgt) => tgt,
    Err(_) => return,
  };

  let this_frame = game.this_frame();
  let mut query = match game.world.query_one::<(
    &mut IsSpectating,
    &IsAlive,
    &mut Spectating,
    &Health,
    &LastActionTime,
  )>(event.entity)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };
  let (spec, alive, target, health, last_action) = match query.get() {
    Some(query) => query,
    None => return,
  };

  if !can_spectate(spec.0, alive.0, health.0, last_action.0, this_frame) {
    game.send_to(
      event.entity,
      s::Error {
        error: ErrorType::IdleRequiredBeforeSpectate,
      },
    );
    return;
  }

  target.0 = spectate_target(event.entity, target.0, tgt, game);
  spec.0 = true;

  drop(query);

  let was_alive = std::mem::replace(
    &mut game.world.get_mut::<IsAlive>(event.entity).unwrap().0,
    false,
  );

  game.dispatch(PlayerSpectate {
    player: event.entity,
    was_alive,
  });
}

#[handler]
fn on_upgrade_command(event: &PacketEvent<Command>, game: &mut AirmashGame) {
  use crate::protocol::server::PlayerUpgrade;

  if event.packet.com != "upgrade" {
    return;
  }

  let (upgrades, prev, _) = match game
    .world
    .query_one_mut::<(&mut Upgrades, &mut PrevUpgrades, &IsPlayer)>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let upgrade_num: u8 = match event.packet.data.to_str_lossy().parse() {
    Ok(upgrade_num) => upgrade_num,
    Err(_) => return,
  };

  if upgrades.unused == 0 {
    return;
  }

  let (count, ty) = match upgrade_num {
    1 => (&mut upgrades.speed, UpgradeType::Speed),
    2 => (&mut upgrades.defense, UpgradeType::Defense),
    3 => (&mut upgrades.energy, UpgradeType::Energy),
    4 => (&mut upgrades.missile, UpgradeType::Missile),
    _ => return,
  };

  if *count == 5 {
    return;
  }

  *count += 1;
  upgrades.unused -= 1;
  prev.0 = *upgrades;

  let packet = PlayerUpgrade {
    upgrades: upgrades.unused,
    ty,
    speed: upgrades.speed,
    defense: upgrades.defense,
    energy: upgrades.energy,
    missile: upgrades.missile,
  };

  game.force_update(event.entity);
  game.send_to(event.entity, packet);
}
