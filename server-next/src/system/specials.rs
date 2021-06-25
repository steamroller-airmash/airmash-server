use airmash_protocol::{KeyCode, PlaneType};

use crate::{
  component::*,
  event::{EventBoost, KeyEvent},
  resource::Config,
  AirmashWorld,
};

pub fn update(game: &mut AirmashWorld) {
  kill_predator_boost_when_out_of_energy(game);
}

fn kill_predator_boost_when_out_of_energy(game: &mut AirmashWorld) {
  let mut query = game
    .world
    .query::<(&Energy, &PlaneType, &mut SpecialActive)>()
    .with::<IsPlayer>();

  let config = game.resources.read::<Config>();

  let mut events = vec![];

  for (ent, (energy, plane, active)) in query.iter() {
    if *plane != PlaneType::Predator {
      continue;
    }

    if energy.0 >= config.planes.predator.energy_regen {
      continue;
    }

    active.0 = false;
    events.push(EventBoost {
      player: ent,
      boosting: false,
    });
  }

  drop(query);
  drop(config);

  for event in events {
    game.dispatch(event);
  }
}

/// Special handling for tracking predator boosts.
#[handler]
fn track_predator_boost(event: &KeyEvent, game: &mut AirmashWorld) {
  match event.key {
    KeyCode::Up | KeyCode::Down | KeyCode::Special => (),
    _ => return,
  }

  let pred_regen = {
    let config = game.resources.read::<Config>();
    config.planes.predator.energy_regen
  };

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
    if energy.0 < pred_regen {
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
