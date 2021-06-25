use airmash_protocol::server::PlayerUpdate;

use crate::component::*;
use crate::event::PlayerJoin;
use crate::protocol::{PlaneType, PowerupType, Upgrades as ServerUpgrades, Vector2};
use crate::resource::*;
use crate::util::get_current_clock;
use crate::AirmashWorld;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::time::Duration;

pub fn frame_update(game: &mut AirmashWorld) {
  update_player_positions(game);
  send_update_packets(game);
}

fn update_player_positions(game: &mut AirmashWorld) {
  let query = game
    .world
    .query_mut::<(
      &mut Position,
      &mut Rotation,
      &mut Velocity,
      &KeyState,
      &Upgrades,
      Option<&Powerup>,
      &PlaneType,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  let config = game
    .resources
    .get::<Config>()
    .expect("Missing game config!");
  let this_frame = game.resources.get::<ThisFrame>().unwrap();
  let last_frame = game.resources.get::<LastFrame>().unwrap();
  let delta = crate::util::convert_time(this_frame.0 - last_frame.0);

  for (_entity, (pos, rot, vel, keystate, upgrades, powerup, plane, alive)) in query {
    if !alive.0 {
      continue;
    }

    let mut movement_angle = None;
    let info = &config.planes[*plane];
    let boost_factor = match keystate.boost(plane) {
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
      vel.0 += Vector2::new(mult * angle.sin(), mult * -angle.cos());
    }

    let old_vel = vel.0;
    let speed = vel.norm();
    let mut max_speed = info.max_speed * boost_factor;
    let min_speed = info.min_speed;

    if upgrades.speed != 0 {
      max_speed *= config.upgrades.speed.factor[upgrades.speed as usize];
    }

    if powerup.map(|x| x.ty) == Some(PowerupType::Inferno) {
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

    let bound = Vector2::new(16352.0, 8160.0);
    if pos.x.abs() > bound.x {
      pos.x = pos.x.signum() * bound.x;
    }
    if pos.y.abs() > bound.y {
      pos.y = pos.y.signum() * bound.y;
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
      Option<&Powerup>,
      &mut LastUpdateTime,
      &Team,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  let this_frame = game.resources.read::<ThisFrame>().0;

  for (ent, (pos, rot, vel, plane, keystate, upgrades, powerup, last_update, team, alive)) in
    query.iter()
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
      shield: powerup
        .map(|x| x.ty == PowerupType::Shield)
        .unwrap_or(false),
      inferno: powerup
        .map(|x| x.ty == PowerupType::Inferno)
        .unwrap_or(false),
    };

    let state = keystate.to_server(plane);

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
