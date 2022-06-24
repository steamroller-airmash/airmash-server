use crate::component::*;
use crate::config::{MissilePrototypeRef, MobPrototypeRef};
use crate::event::EventHorizon;
use crate::AirmashGame;

#[handler]
fn send_missile_update(event: &EventHorizon, game: &mut AirmashGame) {
  use crate::protocol::server::MobUpdate;

  if !event.in_horizon {
    return;
  }

  let clock = crate::util::get_current_clock(game);

  let (&pos, &vel, &accel, &missile, _) = match game.world.query_one_mut::<(
    &Position,
    &Velocity,
    &Accel,
    &MissilePrototypeRef,
    &IsMissile,
  )>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  game.send_to(
    event.player,
    MobUpdate {
      id: event.entity.id() as _,
      clock,
      ty: missile.server_type,
      pos: pos.into(),
      speed: vel.into(),
      accel: accel.into(),
      max_speed: missile.max_speed,
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
    .query_one_mut::<(&Position, &MobPrototypeRef, &IsMob)>(event.entity)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  game.send_to(
    event.player,
    MobUpdateStationary {
      id: event.entity.id() as _,
      ty: mob.server_type,
      pos: pos.into(),
    },
  );
}

#[handler]
fn send_horizon_packet(event: &EventHorizon, game: &mut AirmashGame) {
  use crate::protocol::{server as s, LeaveHorizonType};

  if event.in_horizon {
    return;
  }

  if game.world.get::<IsPlayer>(event.player).is_err() {
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
