use airmash_protocol::server::PlayerUpdate;
use airmash_protocol::MobType;
use nalgebra::vector;

use crate::component::*;
use crate::event::PlayerJoin;
use crate::protocol::{PlaneType, Upgrades as ServerUpgrades, Vector2};
use crate::resource::*;
use crate::util::get_current_clock;
use crate::AirmashWorld;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::time::Duration;

pub fn update(game: &mut AirmashWorld) {
  update_player_positions(game);
  update_missile_positions(game);
  send_update_packets(game);
}

fn update_player_positions(game: &mut AirmashWorld) {
  let config = game.resources.read::<Config>();
  let delta = game.frame_delta();

  let query = game
    .world
    .query_mut::<(
      &mut Position,
      &mut Rotation,
      &mut Velocity,
      &KeyState,
      &Upgrades,
      &Powerup,
      &PlaneType,
      &SpecialActive,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  for (_entity, (pos, rot, vel, keystate, upgrades, powerup, plane, active, alive)) in query {
    if !alive.0 {
      continue;
    }

    let mut movement_angle = None;
    let info = &config.planes[*plane];
    let boost_factor = match *plane == PlaneType::Predator && active.0 {
      true => info.boost_factor,
      false => 1.0,
    };

    if keystate.strafe(plane) {
      if keystate.left {
        movement_angle = Some(rot.0 - FRAC_PI_2);
      }
      if keystate.right {
        movement_angle = Some(rot.0 + FRAC_PI_2);
      }
    } else {
      if keystate.left {
        rot.0 -= delta * info.turn_factor;
      }
      if keystate.right {
        rot.0 += delta * info.turn_factor;
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
      let mult = info.accel_factor * delta * boost_factor;
      vel.0 += vector![mult * angle.sin(), mult * -angle.cos()];
    }

    let old_vel = vel.0;
    let speed = vel.norm();
    let mut max_speed = info.max_speed * boost_factor;
    let min_speed = info.min_speed;

    if upgrades.speed != 0 {
      max_speed *= config.upgrades.speed.factor[upgrades.speed as usize];
    }

    if powerup.inferno() {
      max_speed *= info.inferno_factor;
    }

    if keystate.flagspeed {
      max_speed = info.flag_speed;
    }

    if speed > max_speed {
      vel.0 *= max_speed / speed;
    } else {
      if vel.x.abs() > min_speed || vel.y.abs() > min_speed {
        vel.0 *= 1.0 - info.brake_factor * delta;
      } else {
        vel.0 = Vector2::default();
      }
    }

    pos.0 += old_vel * delta + (vel.0 - old_vel) * delta * 0.5;
    rot.0 = (rot.0 % TAU + TAU) % TAU;

    let bound = vector![16352.0, 8160.0];
    if pos.x.abs() > bound.x {
      pos.x = pos.x.signum() * bound.x;
    }
    if pos.y.abs() > bound.y {
      pos.y = pos.y.signum() * bound.y;
    }
  }
}

fn update_missile_positions(game: &mut AirmashWorld) {
  let config = game.resources.read::<Config>();
  let delta = game.frame_delta();

  let mut query = game
    .world
    .query::<(&mut Position, &mut Velocity, &MobType)>()
    .with::<IsMissile>();

  for (_, (pos, vel, mob)) in query.iter() {
    let info = config.mobs[*mob]
      .missile
      .expect("Missile had invalid mob type");

    let oldvel = vel.0;
    vel.0 += vel.normalize() * info.accel * delta;

    let speed = vel.norm();
    if speed > info.max_speed {
      vel.0 *= info.max_speed / speed;
    }

    pos.0 += oldvel * delta + (vel.0 - oldvel) * delta * 0.5;

    let bounds = vector![16384.0, 8192.0];
    let size = bounds * 2.0;

    if pos.x.abs() > bounds.x {
      pos.x -= pos.x.signum() * size.x;
    }
    if pos.y.abs() > bounds.y {
      pos.y -= pos.y.signum() * size.x;
    }
  }
}

fn send_update_packets(game: &mut AirmashWorld) {
  let clock = get_current_clock(game);

  let mut query = game
    .world
    .query::<(
      &Position,
      &Rotation,
      &Velocity,
      &PlaneType,
      &KeyState,
      &Upgrades,
      &Powerup,
      &mut LastUpdateTime,
      &Team,
      &SpecialActive,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  let this_frame = game.resources.read::<ThisFrame>().0;

  for (
    ent,
    (pos, rot, vel, plane, keystate, upgrades, powerup, last_update, team, active, alive),
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
      shield: powerup.shield(),
      inferno: powerup.inferno(),
    };

    let mut state = keystate.to_server(plane);
    if *plane == PlaneType::Predator {
      state.boost = active.0;
    }

    let packet = PlayerUpdate {
      clock,
      id: ent.id() as u16,
      keystate: state,
      pos: pos.0,
      rot: rot.0,
      speed: vel.0,
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
fn add_required_components(event: &PlayerJoin, game: &mut AirmashWorld) {
  let start_time = game.resources.read::<StartTime>().0;
  let _ = game
    .world
    .insert_one(event.player, LastUpdateTime(start_time));
}