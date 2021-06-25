use airmash_protocol::{KeyCode, PlaneType};

use crate::{
  component::*,
  consts::*,
  event::{EventBoost, EventStealth, KeyEvent},
  resource::{Config, StartTime, ThisFrame},
  AirmashWorld,
};

/// Update the keystate component when a new key event comes in
#[handler(priority = crate::priority::HIGH)]
fn update_keystate(event: &KeyEvent, game: &mut AirmashWorld) {
  let (keystate, ..) = match game
    .world
    .query_one_mut::<(&mut KeyState, &IsPlayer)>(event.player)
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
}

/// Special handling for tracking predator boosts.
#[handler]
fn track_predator_boost(event: &KeyEvent, game: &mut AirmashWorld) {
  match event.key {
    KeyCode::Up | KeyCode::Down | KeyCode::Special => (),
    _ => return,
  }

  let (keystate, plane, energy, active, ..) = match game.world.query_one_mut::<(
    &KeyState,
    &PlaneType,
    &Energy,
    &mut SpecialActive,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if *plane != PlaneType::Predator {
    return;
  }

  if !keystate.special {
    if active.0 {
      active.0 = false;
      game.dispatch(EventBoost {
        player: event.player,
        boosting: false,
      });
    }
    return;
  }

  if active.0 {
    // If we don't have the energy to perform a boost then boost is disabled
    if energy.0 < PREDATOR_SPECIAL_REGEN {
      active.0 = false;
      game.dispatch(EventBoost {
        player: event.player,
        boosting: false,
      });
      return;
    }

    // No boosting occurs if neither the up or down keys are pressed
    if !keystate.up && !keystate.down {
      active.0 = false;
      game.dispatch(EventBoost {
        player: event.player,
        boosting: false,
      });
      return;
    }

    // ... Otherwise we continue boosting
  } else {
    if energy.0 < PREDATOR_SPECIAL_REGEN {
      return;
    }

    // Player pressed a key so now we start boosting
    if keystate.up || keystate.down {
      active.0 = true;
      game.dispatch(EventBoost {
        player: event.player,
        boosting: true,
      });
      return;
    }
  }
}

/// If a key event would cause a plane to perform its special then emit the
/// correct event for that special.
///
/// # Note
/// This only applies for specials that are edge-triggered (i.e. by the button
/// being pressed). If the special would continue to be triggered while the
/// special button is held then we just mark it as active here.
///
/// Predator, prowler, and tornado specials are edge-triggered while mohawk, and
/// goliath specials are level triggered.
#[handler]
fn send_special_event(event: &KeyEvent, game: &mut AirmashWorld) {
  if event.key != KeyCode::Special {
    return;
  }

  let (&plane, energy, last_special, active, ..) = match game.world.query_one_mut::<(
    &PlaneType,
    &mut Energy,
    &mut LastSpecialTime,
    &mut SpecialActive,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let config = game.resources.read::<Config>();
  let this_frame = *game.resources.read::<ThisFrame>();

  let time_since_last = this_frame.0 - last_special.0;

  match plane {
    PlaneType::Mohawk => active.0 = event.state,
    // Predator boost behaviour is somewhat complicated so it's handled in track_predator_boost
    PlaneType::Predator => (),
    PlaneType::Prowler => {
      if !event.state {
        return;
      }

      if active.0 {
        if time_since_last < PROWLER_SPECIAL_DELAY {
          return;
        }

        if energy.0 < PROWLER_SPECIAL_ENERGY {
          return;
        }

        last_special.0 = this_frame.0;
        energy.0 -= PROWLER_SPECIAL_ENERGY;
      }

      active.0 = !active.0;
      let evt = EventStealth {
        player: event.player,
        stealthed: active.0,
      };

      drop(config);
      game.dispatch(evt);
    }
    PlaneType::Goliath => active.0 = event.state,
    PlaneType::Tornado => {
      todo!()
    }
  }
}

/// Force the physics system to emit a PlayerUpdate packet ASAP when the player
/// presses a key that changes the plane's direction or speed.
#[handler]
fn force_update_packet(event: &KeyEvent, game: &mut AirmashWorld) {
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
