use airmash_protocol::{KeyCode, PlaneType};
use hecs::Entity;
use nalgebra::vector;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::{
  component::*,
  event::KeyEvent,
  resource::{Config, StartTime, ThisFrame},
  AirmashGame, FireMissileInfo,
};

pub fn update(game: &mut AirmashGame) {
  fire_missiles(game);
}

fn fire_missiles(game: &mut AirmashGame) {
  let config = game.resources.read::<Config>();
  let this_frame = game.this_frame();

  let mut query = game
    .world
    .query::<(
      &KeyState,
      &LastFireTime,
      &mut Energy,
      &PlaneType,
      &Powerup,
      &IsAlive,
      &mut MissileFiringSide,
    )>()
    .with::<IsPlayer>();

  let mut events: Vec<(Entity, SmallVec<[FireMissileInfo; 3]>)> = Vec::new();
  for (ent, (keystate, last_fire, energy, plane, powerup, alive, side)) in query.iter() {
    let info = &config.planes[*plane];

    if !alive.0
      || !keystate.fire
      || this_frame - last_fire.0 < info.fire_delay
      || energy.0 < info.fire_energy
    {
      continue;
    }

    energy.0 -= info.fire_energy;

    let side_mult = match std::mem::replace(side, side.reverse()) {
      MissileFiringSide::Left => -1.0,
      MissileFiringSide::Right => 1.0,
    };
    let hor_offset = info.missile_offset.y * side_mult;

    let mut missile_info = smallvec![FireMissileInfo {
      pos_offset: vector![hor_offset, info.missile_offset.x],
      rot_offset: 0.0,
      ty: info.missile_type
    }];

    if powerup.inferno() {
      missile_info.push(FireMissileInfo {
        pos_offset: vector![
          info.missile_inferno_offset_x + hor_offset,
          info.missile_inferno_offset_y
        ],
        rot_offset: -info.missile_inferno_angle,
        ty: info.missile_type,
      });
      missile_info.push(FireMissileInfo {
        pos_offset: vector![
          -info.missile_inferno_offset_x + hor_offset,
          info.missile_inferno_offset_y
        ],
        rot_offset: info.missile_inferno_angle,
        ty: info.missile_type,
      });
    }

    events.push((ent, missile_info));
  }

  drop(config);
  drop(query);

  for (ent, missiles) in events {
    let _ = game.fire_missiles(ent, &missiles);
  }
}

/// Update the keystate component when a new key event comes in
#[handler(priority = crate::priority::HIGH)]
fn update_keystate(event: &KeyEvent, game: &mut AirmashGame) {
  let this_frame = game.resources.read::<ThisFrame>().0;

  let (keystate, last_action, ..) = match game
    .world
    .query_one_mut::<(&mut KeyState, &mut LastActionTime, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  match event.key {
    KeyCode::Up => keystate.up = event.state,
    KeyCode::Down => keystate.down = event.state,
    KeyCode::Left => keystate.left = event.state,
    KeyCode::Right => keystate.right = event.state,
    KeyCode::Fire => keystate.fire = event.state,
    KeyCode::Special => keystate.special = event.state,
  }

  last_action.0 = this_frame;
}

/// Force the physics system to emit a PlayerUpdate packet ASAP when the player
/// presses a key that changes the plane's direction or speed.
#[handler]
fn force_update_packet(event: &KeyEvent, game: &mut AirmashGame) {
  // Other keys don't force updates
  match event.key {
    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => (),
    _ => return,
  }

  let (last_update, ..) = match game
    .world
    .query_one_mut::<(&mut LastUpdateTime, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  last_update.0 = game.resources.read::<StartTime>().0;
}
