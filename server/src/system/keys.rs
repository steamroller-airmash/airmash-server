use crate::component::*;
use crate::config::PlanePrototypeRef;
use crate::event::KeyEvent;
use crate::protocol::KeyCode;
use crate::resource::{Config, StartTime, ThisFrame};
use crate::AirmashGame;

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
      &PlanePrototypeRef,
      &Effects,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  let mut events = Vec::new();
  for (ent, (keystate, last_fire, energy, plane, effects, alive)) in query.iter() {
    if !alive.0
      || !keystate.fire
      || this_frame - last_fire.0 < plane.fire_delay
      || energy.0 < plane.fire_energy
    {
      continue;
    }

    energy.0 -= plane.fire_energy;

    let mut count = 1;
    if effects.has_inferno() {
      count = count * 2 + 1;
    }

    events.push((ent, count, plane.missile));
  }

  drop(config);
  drop(query);

  for (ent, missiles, ty) in events {
    let _ = game.fire_missiles_count(ent, missiles, ty);
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
