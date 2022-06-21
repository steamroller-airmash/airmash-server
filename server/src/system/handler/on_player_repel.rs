use crate::component::*;
use crate::config::{MissilePrototypeRef, PlanePrototypeRef};
use crate::event::{EventStealth, PlayerRepel};
use crate::AirmashGame;

#[handler]
fn send_packet(event: &PlayerRepel, game: &mut AirmashGame) {
  use crate::protocol::server as s;

  let clock = crate::util::get_current_clock(game);

  let (&pos, &rot, &vel, &energy, &regen, _) = match game.world.query_one_mut::<(
    &Position,
    &Rotation,
    &Velocity,
    &Energy,
    &EnergyRegen,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let mut players = Vec::new();
  for player in event.repelled_players.iter().copied() {
    let (
      &pos,
      &rot,
      &vel,
      keystate,
      health,
      health_regen,
      energy,
      energy_regen,
      plane,
      active,
      effects,
      ..,
    ) = match game.world.query_one_mut::<(
      &Position,
      &Rotation,
      &Velocity,
      &KeyState,
      &Health,
      &HealthRegen,
      &Energy,
      &EnergyRegen,
      &PlanePrototypeRef,
      &SpecialActive,
      &Effects,
      &IsPlayer,
    )>(player)
    {
      Ok(query) => query,
      Err(_) => continue,
    };

    players.push(s::EventRepelPlayer {
      id: player.id() as _,
      keystate: keystate.to_server(plane, active, effects),
      health: health.0,
      health_regen: health_regen.0,
      energy: energy.0,
      energy_regen: energy_regen.0,
      pos: pos.0,
      rot: rot.0,
      speed: vel.0,
    });
  }

  let mut missiles = Vec::new();
  for missile in event.repelled_missiles.iter().copied() {
    let mut query = match game
      .world
      .query_one::<(&Position, &Velocity, &Accel, &MissilePrototypeRef)>(missile)
      .map(|q| q.with::<IsMissile>())
    {
      Ok(query) => query,
      Err(_) => continue,
    };

    let (pos, vel, accel, &mob, ..) = match query.get() {
      Some(value) => value,
      None => continue,
    };

    missiles.push(s::EventRepelMob {
      id: missile.id() as _,
      pos: pos.0,
      accel: accel.0,
      speed: vel.0,
      max_speed: mob.max_speed,
      ty: mob.server_type,
    });
  }

  let packet = s::EventRepel {
    clock,
    id: event.player.id() as _,
    energy: energy.0,
    energy_regen: regen.0,
    pos: pos.0,
    rot: rot.0,
    speed: vel.0,
    players,
    mobs: missiles,
  };

  game.send_to_visible(packet.pos, packet);
}

#[handler(priority = crate::priority::MEDIUM)]
fn repel_players(event: &PlayerRepel, game: &mut AirmashGame) {
  let (&player_pos, _) = match game
    .world
    .query_one_mut::<(&Position, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  for player in event.repelled_players.iter().copied() {
    let (pos, vel, ..) = match game
      .world
      .query_one_mut::<(&Position, &mut Velocity, &IsPlayer)>(player)
    {
      Ok(query) => query,
      Err(_) => continue,
    };

    let dir = (pos.0 - player_pos.0).normalize();
    // TODO: The reflect speed constant didn't do what I wanted here so I've bumped
    //       it up to 10. This should probably be reevaluated at some point.
    vel.0 = dir * 10.0;
  }
}

#[handler(priority = crate::priority::MEDIUM)]
fn repel_missiles(event: &PlayerRepel, game: &mut AirmashGame) {
  let (&player_pos, &player_team, _) = match game
    .world
    .query_one_mut::<(&Position, &Team, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  for missile in event.repelled_missiles.iter().copied() {
    let mut query = match game
      .world
      .query_one::<(
        &Position,
        &mut Velocity,
        &mut Accel,
        &mut Team,
        &mut Owner,
        &mut MissileTrajectory,
        &MissilePrototypeRef,
      )>(missile)
      .map(|q| q.with::<IsMissile>())
    {
      Ok(query) => query,
      Err(_) => continue,
    };

    let (pos, vel, accel, team, owner, traj, &mob, ..) = match query.get() {
      Some(value) => value,
      None => continue,
    };

    let total_dist = (traj.start - pos.0).norm();
    traj.start = pos.0;
    traj.maxdist -= total_dist;

    let dir = (pos.0 - player_pos.0).normalize();

    vel.0 = dir * vel.0.norm();
    accel.0 = (-accel.normalize() + dir).normalize() * mob.accel;
    team.0 = player_team.0;
    owner.0 = event.player;
  }
}

#[handler]
fn decloak_prowlers(event: &PlayerRepel, game: &mut AirmashGame) {
  if game.world.get::<IsPlayer>(event.player).is_err() {
    return;
  }

  for player in event.repelled_players.iter().copied() {
    let (&plane, active, _) = match game
      .world
      .query_one_mut::<(&PlanePrototypeRef, &mut SpecialActive, &IsPlayer)>(player)
    {
      Ok(query) => query,
      Err(_) => continue,
    };

    if !active.0 || !plane.special.is_stealth() {
      continue;
    }

    game.dispatch(EventStealth {
      player,
      stealthed: false,
    });
  }
}
