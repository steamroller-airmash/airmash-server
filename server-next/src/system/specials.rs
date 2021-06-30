use hecs::Entity;
use smallvec::SmallVec;

use crate::{
  component::*,
  consts::*,
  event::{EventBoost, EventStealth, KeyEvent, PlayerRepel},
  protocol::KeyCode,
  resource::{
    collision::{MissileCollideDb, PlayerCollideDb},
    Config,
  },
  AirmashWorld, FireMissileInfo,
};

pub fn update(game: &mut AirmashWorld) {
  kill_predator_boost_when_out_of_energy(game);
  tornado_special_fire(game);
  goliath_repel(game);
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

fn tornado_special_fire(game: &mut AirmashWorld) {
  let config = game.resources.read::<Config>();
  let this_frame = game.this_frame();

  let mut query = game
    .world
    .query::<(&KeyState, &LastFireTime, &mut Energy, &PlaneType, &Powerup)>()
    .with::<IsPlayer>();

  let mut events: Vec<(Entity, SmallVec<[FireMissileInfo; 5]>)> = Vec::new();
  for (ent, (keystate, last_fire, energy, &plane, powerup)) in query.iter() {
    if plane != PlaneType::Tornado {
      continue;
    }

    if !keystate.special {
      continue;
    }

    let ref info = config.planes[plane];
    if this_frame - last_fire.0 < info.fire_delay {
      continue;
    }

    if energy.0 < TORNADO_SPECIAL_ENERGY {
      continue;
    }

    energy.0 -= TORNADO_SPECIAL_ENERGY;

    let mut missiles = SmallVec::new();
    if powerup.inferno() {
      missiles.extend_from_slice(&TORNADO_INFERNO_MISSILE_DETAILS[..]);
    } else {
      missiles.extend_from_slice(&TORNADO_MISSILE_DETAILS[..]);
    }

    events.push((ent, missiles));
  }

  drop(config);
  drop(query);

  for (ent, missiles) in events {
    let _ = game.fire_missiles(ent, &missiles);
  }
}

fn goliath_repel(game: &mut AirmashWorld) {
  let this_frame = game.this_frame();
  let query = game
    .world
    .query_mut::<(
      &Position,
      &Team,
      &mut Energy,
      &KeyState,
      &mut LastSpecialTime,
      &PlaneType,
    )>()
    .with::<IsPlayer>();

  let mut players = SmallVec::<[_; 16]>::new();
  for (ent, (pos, team, energy, keystate, last_special, &plane)) in query {
    if plane != PlaneType::Goliath || !keystate.special {
      continue;
    }

    if this_frame - last_special.0 < GOLIATH_SPECIAL_INTERVAL {
      continue;
    }

    if energy.0 < GOLIATH_SPECIAL_ENERGY {
      continue;
    }

    last_special.0 = this_frame;
    energy.0 -= GOLIATH_SPECIAL_ENERGY;
    players.push((ent, pos.0, team.0));
  }

  let mut events = SmallVec::<[_; 16]>::new();
  let player_db = game.resources.read::<PlayerCollideDb>();
  let missile_db = game.resources.read::<MissileCollideDb>();
  for (player, pos, team) in players {
    let mut event = PlayerRepel {
      player,
      repelled_players: SmallVec::new(),
      repelled_missiles: SmallVec::new(),
    };

    player_db.query(
      pos,
      GOLIATH_SPECIAL_RADIUS_PLAYER,
      Some(team),
      &mut event.repelled_players,
    );
    missile_db.query(
      pos,
      GOLIATH_SPECIAL_RADIUS_MISSILE,
      Some(team),
      &mut event.repelled_missiles,
    );

    event.repelled_players.sort_unstable();
    event.repelled_players.dedup();

    event.repelled_missiles.sort_unstable();
    event.repelled_missiles.dedup();

    events.push(event);
  }

  drop(player_db);
  drop(missile_db);

  game.dispatch_many(events);
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

#[handler]
fn prowler_cloak(event: &KeyEvent, game: &mut AirmashWorld) {
  if event.key != KeyCode::Special {
    return;
  }

  let this_frame = game.this_frame();

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

  // Prowlers only change stealth state when shift is pressed
  if plane != PlaneType::Prowler || !event.state {
    return;
  }

  if active.0 {
    if this_frame - last_special.0 < PROWLER_SPECIAL_DELAY {
      return;
    }

    if energy.0 < PROWLER_SPECIAL_ENERGY {
      return;
    }

    last_special.0 = this_frame;
    energy.0 -= PROWLER_SPECIAL_ENERGY;
  }

  active.0 = !active.0;
  let evt = EventStealth {
    player: event.player,
    stealthed: active.0,
  };

  game.dispatch(evt);
}
