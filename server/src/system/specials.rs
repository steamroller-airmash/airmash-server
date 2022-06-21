use smallvec::SmallVec;

use crate::component::*;
use crate::config::PlanePrototypeRef;
use crate::consts::*;
use crate::event::{
  EventBoost, EventStealth, KeyEvent, PlayerFire, PlayerMissileCollision, PlayerRepel,
};
use crate::protocol::KeyCode;
use crate::resource::collision::{LayerSpec, MissileCollideDb, PlayerCollideDb};
use crate::resource::Config;
use crate::AirmashGame;

pub fn update(game: &mut AirmashGame) {
  kill_predator_boost_when_out_of_energy(game);
  tornado_special_fire(game);
  goliath_repel(game);
}

fn kill_predator_boost_when_out_of_energy(game: &mut AirmashGame) {
  let mut query = game
    .world
    .query::<(&Energy, &PlanePrototypeRef, &mut SpecialActive, &IsAlive)>()
    .with::<IsPlayer>();

  let mut events = vec![];

  for (ent, (energy, plane, active, alive)) in query.iter() {
    let boost = match plane.special.as_boost() {
      Some(boost) => boost,
      _ => continue,
    };

    if energy.0 >= boost.cost || !alive.0 {
      continue;
    }

    active.0 = false;
    events.push(EventBoost {
      player: ent,
      boosting: false,
    });
  }

  drop(query);

  for event in events {
    game.dispatch(event);
  }
}

fn tornado_special_fire(game: &mut AirmashGame) {
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
  for (ent, (keystate, last_fire, energy, &plane, powerup, alive)) in query.iter() {
    if !keystate.special || !alive.0 {
      continue;
    }

    let multishot = match plane.special.as_multishot() {
      Some(multishot) => multishot,
      _ => continue,
    };

    if this_frame - last_fire.0 < multishot.delay || energy.0 < multishot.cost {
      continue;
    }

    energy.0 -= multishot.cost;

    let mut missiles = SmallVec::<[_; 5]>::new();
    // FIXME: This currently ignores the multishot.count property.
    if powerup.has_inferno() {
      missiles.extend_from_slice(&tornado_inferno_missile_details(multishot.missile))
    } else {
      missiles.extend_from_slice(&tornado_missile_details(multishot.missile));
    }

    events.push((ent, missiles));
  }

  drop(config);
  drop(query);

  for (ent, missiles) in events {
    let _ = game.fire_missiles(ent, &missiles);
  }
}

fn goliath_repel(game: &mut AirmashGame) {
  let this_frame = game.this_frame();
  let query = game
    .world
    .query_mut::<(
      &Position,
      &Team,
      &mut Energy,
      &KeyState,
      &mut LastSpecialTime,
      &PlanePrototypeRef,
      &IsAlive,
    )>()
    .with::<IsPlayer>();

  let mut players = SmallVec::<[_; 16]>::new();
  for (ent, (pos, team, energy, keystate, last_special, &plane, alive)) in query {
    let repel = match plane.special.as_repel() {
      Some(repel) => repel,
      _ => continue,
    };

    if !keystate.special
      || !alive.0
      || this_frame - last_special.0 < repel.delay
      || energy.0 < repel.cost
    {
      continue;
    }

    last_special.0 = this_frame;
    energy.0 -= repel.cost;
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

    // FIXME: These should use the radius parameter from the prototype.
    player_db.query(
      pos,
      GOLIATH_SPECIAL_RADIUS_PLAYER,
      LayerSpec::Exclude(team),
      &mut event.repelled_players,
    );
    missile_db.query(
      pos,
      GOLIATH_SPECIAL_RADIUS_MISSILE,
      LayerSpec::Exclude(team),
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
fn track_predator_boost(event: &KeyEvent, game: &mut AirmashGame) {
  match event.key {
    KeyCode::Up | KeyCode::Down | KeyCode::Special => (),
    _ => return,
  }

  let (keystate, plane, energy, active, ..) = match game.world.query_one_mut::<(
    &KeyState,
    &PlanePrototypeRef,
    &Energy,
    &mut SpecialActive,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let boost = match plane.special.as_boost() {
    Some(boost) => boost,
    _ => return,
  };

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
    }

    // ... Otherwise we continue boosting
  } else {
    if energy.0 < boost.cost {
      return;
    }

    // Player pressed a key so now we start boosting
    if keystate.up || keystate.down {
      active.0 = true;
      game.dispatch(EventBoost {
        player: event.player,
        boosting: true,
      });
    }
  }
}

#[handler]
fn prowler_cloak(event: &KeyEvent, game: &mut AirmashGame) {
  // Prowlers only change stealth state when shift is pressed
  if event.key != KeyCode::Special {
    return;
  }

  let this_frame = game.this_frame();

  let (&plane, energy, last_special, active, alive, _) = match game.world.query_one_mut::<(
    &PlanePrototypeRef,
    &mut Energy,
    &mut LastSpecialTime,
    &mut SpecialActive,
    &IsAlive,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let stealth = match plane.special.as_stealth() {
    Some(stealth) => stealth,
    _ => return,
  };

  if !event.state || !alive.0 {
    return;
  }

  if !active.0 {
    if this_frame - last_special.0 < stealth.delay {
      return;
    }

    if energy.0 < stealth.cost {
      return;
    }

    last_special.0 = this_frame;
    energy.0 -= stealth.cost;
  }

  let evt = EventStealth {
    player: event.player,
    stealthed: !active.0,
  };

  game.dispatch(evt);
}

#[handler]
fn prowler_decloak_on_fire(event: &PlayerFire, game: &mut AirmashGame) {
  let (&plane, &active, _) = match game
    .world
    .query_one_mut::<(&PlanePrototypeRef, &SpecialActive, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if !plane.special.is_stealth() || !active.0 {
    return;
  }

  game.dispatch(EventStealth {
    player: event.player,
    stealthed: false,
  });
}

#[handler]
fn prowler_decloak_on_hit(event: &PlayerMissileCollision, game: &mut AirmashGame) {
  for player in event.players.iter().copied() {
    let (&plane, &active, _) = match game
      .world
      .query_one_mut::<(&PlanePrototypeRef, &SpecialActive, &IsPlayer)>(player)
    {
      Ok(query) => query,
      Err(_) => return,
    };

    if !plane.special.is_stealth() || !active.0 {
      return;
    }

    game.dispatch(EventStealth {
      player,
      stealthed: false,
    });
  }
}

#[handler]
fn update_mohawk_on_strafe(event: &KeyEvent, game: &mut AirmashGame) {
  if event.key != KeyCode::Special {
    return;
  }

  let start_time = game.resources.read::<crate::resource::StartTime>().0;

  let (plane, last_update, _) = match game
    .world
    .query_one_mut::<(&PlanePrototypeRef, &mut LastUpdateTime, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if !plane.special.is_strafe() {
    return;
  }

  last_update.0 = start_time;
}
