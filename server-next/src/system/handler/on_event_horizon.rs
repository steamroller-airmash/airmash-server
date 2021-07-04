use airmash_protocol::MobType;

use crate::component::*;
use crate::event::EventHorizon;
use crate::resource::Config;
use crate::AirmashGame;

#[handler]
fn send_missile_update(event: &EventHorizon, game: &mut AirmashGame) {
  use crate::protocol::server::MobUpdate;

  if !event.in_horizon {
    return;
  }

  let clock = crate::util::get_current_clock(game);

  let (&pos, &vel, &accel, &mob, _) =
    match game
      .world
      .query_one_mut::<(&Position, &Velocity, &Accel, &MobType, &IsMissile)>(event.entity)
    {
      Ok(query) => query,
      Err(_) => return,
    };

  let max_speed = {
    let config = game.resources.read::<Config>();

    let info = match config.mobs[mob].missile {
      Some(ref info) => info,
      None => return,
    };

    info.max_speed
  };

  game.send_to(
    event.player,
    MobUpdate {
      id: event.entity.id() as _,
      clock,
      ty: mob,
      pos: pos.0,
      speed: vel.0,
      accel: accel.0,
      max_speed,
    },
  );
}

#[handler]
fn send_mob_update(event: &EventHorizon, game: &mut AirmashGame) {
  use crate::protocol::server::MobUpdateStationary;

  if !event.in_horizon {
    return;
  }

  let (&pos, &mob, _) = match game
    .world
    .query_one_mut::<(&Position, &MobType, &IsMob)>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  game.send_to(
    event.player,
    MobUpdateStationary {
      id: event.entity.id() as _,
      ty: mob,
      pos: pos.0,
    },
  );
}

#[handler]
fn send_horizon_packet(event: &EventHorizon, game: &mut AirmashGame) {
  use crate::protocol::server as s;
  use crate::protocol::LeaveHorizonType;

  if event.in_horizon {
    return;
  }

  if !game.world.get::<IsPlayer>(event.player).is_ok() {
    return;
  }

  let query = game
    .world
    .query_one_mut::<(Option<&IsPlayer>, Option<&IsMissile>, Option<&IsMob>)>(event.entity);

  let ty = match query {
    Ok((Some(_), None, None)) => LeaveHorizonType::Player,
    Ok((None, Some(_), None)) => LeaveHorizonType::Mob,
    Ok((None, None, Some(_))) => LeaveHorizonType::Mob,
    _ => return,
  };

  game.send_to(
    event.player,
    s::EventLeaveHorizon {
      id: event.entity.id() as _,
      ty,
    },
  );
}
