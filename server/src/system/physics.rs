use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::time::Duration;

use crate::component::*;
use crate::config::{MissilePrototypeRef, PlanePrototypeRef};
use crate::event::PlayerJoin;
use crate::protocol::server::PlayerUpdate;
use crate::protocol::Upgrades as ServerUpgrades;
use crate::resource::*;
use crate::util::{get_current_clock, NalgebraExt};
use crate::{AirmashGame, Vector2};

pub fn update(game: &mut AirmashGame) {
  update_player_positions(game);
  update_spectator_positions(game);
  update_missile_positions(game);
  send_update_packets(game);
}

fn update_player_positions(game: &mut AirmashGame) {
  let delta = game.frame_delta();

  let query = game
    .world
    .query_mut::<(
      &mut Position,
      &mut Rotation,
      &mut Velocity,
      &KeyState,
      &Upgrades,
      &Effects,
      &PlanePrototypeRef,
      &SpecialActive,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  for (_entity, (pos, rot, vel, keystate, upgrades, effects, plane, active, alive)) in query {
    if !alive.0 {
      continue;
    }

    let special = plane.special;

    let boost_factor = match special.as_boost() {
      Some(boost) if active.0 => boost.speedup,
      _ => 1.0,
    };
    let strafe = special.is_strafe() && keystate.special && (keystate.left || keystate.right);

    let mut movement_angle = None;
    if strafe {
      if keystate.left {
        movement_angle = Some(rot.0 - FRAC_PI_2);
      }
      if keystate.right {
        movement_angle = Some(rot.0 + FRAC_PI_2);
      }
    } else {
      if keystate.left {
        rot.0 -= delta * plane.turn_factor;
      }
      if keystate.right {
        rot.0 += delta * plane.turn_factor;
      }
    }

    if keystate.up {
      if let Some(angle) = movement_angle {
        if keystate.right {
          movement_angle = Some(angle - PI * 0.25);
        } else if keystate.left {
          movement_angle = Some(angle + PI * 0.25);
        }
      } else {
        movement_angle = Some(rot.0);
      }
    } else if keystate.down {
      if let Some(angle) = movement_angle {
        if keystate.right {
          movement_angle = Some(angle + PI * 0.25);
        } else if keystate.left {
          movement_angle = Some(angle - PI * 0.25);
        }
      } else {
        movement_angle = Some(rot.0 + PI);
      }
    }

    if let Some(angle) = movement_angle {
      let mult = plane.accel * delta * boost_factor;
      vel.0 += Vector2::new(mult * angle.sin(), mult * -angle.cos());
    }

    let old_vel = vel.0;
    let speed = vel.norm();
    let mut max_speed = plane.max_speed * boost_factor;
    let min_speed = plane.min_speed;

    if upgrades.speed != 0 {
      max_speed *= crate::consts::UPGRADE_MULTIPLIERS[upgrades.speed as usize];
    }

    if effects.has_inferno() {
      max_speed *= plane.inferno_factor;
    }

    if let Some(speed) = effects.fixed_speed() {
      max_speed = speed;
    }

    if speed > max_speed {
      vel.0 *= max_speed / speed;
    } else if vel.x.abs() > min_speed || vel.y.abs() > min_speed {
      vel.0 *= 1.0 - plane.brake * delta;
    } else {
      vel.0 = Vector2::default();
    }

    pos.0 += old_vel * delta + (vel.0 - old_vel) * delta * 0.5;
    rot.0 = (rot.0 % TAU + TAU) % TAU;

    let bound = Vector2::new(16352.0, 8160.0);
    if pos.x.abs() > bound.x {
      pos.x = pos.x.signum() * bound.x;
    }
    if pos.y.abs() > bound.y {
      pos.y = pos.y.signum() * bound.y;
    }
  }
}

fn update_missile_positions(game: &mut AirmashGame) {
  let delta = game.frame_delta();

  let mut query = game
    .world
    .query::<(&mut Position, &mut Velocity, &Accel, &MissilePrototypeRef)>()
    .with::<IsMissile>();

  for (_, (pos, vel, accel, missile)) in query.iter() {
    let oldvel = vel.0;
    vel.0 += accel.0 * delta;

    let speed = vel.norm();
    if speed > missile.max_speed {
      vel.0 *= missile.max_speed / speed;
    }

    pos.0 += oldvel * delta + (vel.0 - oldvel) * delta * 0.5;

    let bounds = Vector2::new(16384.0, 8192.0);
    let size = bounds * 2.0;

    if pos.x.abs() > bounds.x {
      pos.x -= pos.x.signum() * size.x;
    }
    if pos.y.abs() > bounds.y {
      pos.y -= pos.y.signum() * size.x;
    }
  }
}

fn update_spectator_positions(game: &mut AirmashGame) {
  let mut query = game
    .world
    .query::<(&mut Position, &IsSpectating, &Spectating)>()
    .with::<IsPlayer>();

  for (player, (pos, spec, target)) in query.iter() {
    if !spec.0 {
      continue;
    }

    let target = match target.0 {
      Some(target) if target != player => target,
      _ => {
        pos.0 = Vector2::zeros();
        continue;
      }
    };

    pos.0 = match unsafe { game.world.get_unchecked::<Position>(target) } {
      Ok(&pos) => pos.0,
      Err(_) => continue,
    };
  }
}

fn send_update_packets(game: &mut AirmashGame) {
  let clock = get_current_clock(game);

  let mut query = game
    .world
    .query::<(
      &Position,
      &Rotation,
      &Velocity,
      &PlanePrototypeRef,
      &KeyState,
      &Upgrades,
      &Effects,
      &mut LastUpdateTime,
      &Team,
      &SpecialActive,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  let this_frame = game.resources.read::<ThisFrame>().0;

  for (
    ent,
    (pos, rot, vel, plane, keystate, upgrades, effects, last_update, team, active, alive),
  ) in query.iter()
  {
    if !alive.0 {
      continue;
    }

    if this_frame - last_update.0 < Duration::from_secs(1) {
      continue;
    }
    last_update.0 = this_frame;

    let ups = ServerUpgrades {
      speed: upgrades.speed,
      shield: effects.has_shield(),
      inferno: effects.has_inferno(),
    };

    let packet = PlayerUpdate {
      clock,
      id: ent.id() as u16,
      keystate: keystate.to_server(plane, active, effects),
      pos: pos.into(),
      rot: rot.0,
      speed: vel.into(),
      upgrades: ups,
    };

    if keystate.stealthed {
      game.send_to_team_visible(team.0, pos.0, packet);
    } else {
      // game.send_to_all(packet);
      game.send_to_visible(pos.0, packet);
    }
  }
}

#[handler]
fn add_required_components(event: &PlayerJoin, game: &mut AirmashGame) {
  let start_time = game.resources.read::<StartTime>().0;
  let _ = game
    .world
    .insert_one(event.player, LastUpdateTime(start_time));
}
