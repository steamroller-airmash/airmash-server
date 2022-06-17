use airmash::component::*;
use airmash::event::Frame;
use airmash::resource::collision::LayerSpec;
use airmash::AirmashGame;
use smallvec::SmallVec;

use crate::component::*;
use crate::config;
use crate::event::*;
use crate::resource::*;

fn update_flag_positions(game: &mut AirmashGame) {
  let mut query = game
    .world
    .query::<(&mut Position, &FlagCarrier)>()
    .with::<IsFlag>();

  for (_, (pos, carrier)) in query.iter() {
    let carrier = match carrier.0 {
      Some(carrier) => carrier,
      None => continue,
    };

    let mut query = match game.world.query_one::<&Position>(carrier) {
      Ok(query) => query.with::<IsPlayer>(),
      Err(_) => continue,
    };

    if let Some(player_pos) = query.get() {
      pos.0 = player_pos.0;
    }
  }
}

fn capture_flags(game: &mut AirmashGame) {
  let scores = game.resources.read::<GameScores>();
  let this_frame = game.this_frame();
  let mut query = game
    .world
    .query::<(&Position, &Team, &mut FlagCarrier, &mut LastReturnTime)>()
    .with::<IsFlag>();

  let mut events = SmallVec::<[_; 1]>::new();
  for (flag, (pos, team, carrier, last_return)) in query.iter() {
    if carrier.0.is_none() {
      continue;
    }

    let return_pos = config::flag_return_pos(team.0);
    if (return_pos - pos.0).norm() > config::FLAG_RADIUS {
      continue;
    }

    last_return.0 = this_frame;

    events.push(FlagEvent {
      ty: FlagEventType::Capture,
      player: carrier.0.take(),
      flag,
    });
  }

  drop(scores);
  drop(query);

  game.dispatch_many(events);
}

fn return_and_pickup_flags(game: &mut AirmashGame) {
  use airmash::resource::collision::PlayerPosDb;

  // Can't do anything with flags when there is no active game.
  if !game.resources.read::<GameActive>().0 {
    return;
  }

  let player_db = game.resources.read::<PlayerPosDb>();
  let this_frame = game.this_frame();

  let mut query = game
    .world
    .query::<(
      &mut Position,
      &Team,
      &LastDrop,
      &mut LastReturnTime,
      &mut FlagCarrier,
    )>()
    .with::<IsFlag>();

  let mut players = SmallVec::<[_; 4]>::new();
  let mut positions = SmallVec::<[_; 4]>::new();
  let mut events = SmallVec::<[_; 2]>::new();
  for (flag, (pos, team, last_drop, last_return, carrier)) in query.iter() {
    if carrier.0.is_some() || this_frame - last_return.0 < config::FLAG_NO_REGRAB_TIME {
      continue;
    }

    if pos.0 == config::flag_home_pos(team.0) {
      player_db.query(
        pos.0,
        config::FLAG_RADIUS,
        LayerSpec::Exclude(team.0),
        &mut players,
      );
    } else {
      player_db.query(pos.0, config::FLAG_RADIUS, LayerSpec::None, &mut players);
    }

    positions.clear();
    for player in players.drain(..) {
      let mut query = match game.world.query_one::<(&Position, &Team, &IsAlive)>(player) {
        Ok(query) => query.with::<IsPlayer>(),
        Err(_) => continue,
      };

      let (player_pos, player_team, alive) = match query.get() {
        Some(query) => query,
        None => continue,
      };

      if !alive.0 {
        continue;
      }

      if this_frame - last_drop.time < config::FLAG_NO_REGRAB_TIME
        && last_drop.player.map(|e| e == player).unwrap_or(false)
      {
        continue;
      }

      positions.push((player, player_team.0, (player_pos.0 - pos.0).norm_squared()));
    }

    // Explicitly make sure that the closest player to the flag gets priority
    positions.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    if let Some(&(ent, player_team, _)) = positions.first() {
      if team.0 == player_team {
        last_return.0 = this_frame;
        events.push(FlagEvent {
          ty: FlagEventType::Return,
          player: Some(ent),
          flag,
        });
      } else {
        carrier.0 = Some(ent);
        events.push(FlagEvent {
          ty: FlagEventType::PickUp,
          player: Some(ent),
          flag,
        });
      }
    }
  }

  drop(query);
  drop(player_db);

  game.dispatch_many(events);
}

#[handler]
pub fn tick_updates(_: &Frame, game: &mut AirmashGame) {
  update_flag_positions(game);
  capture_flags(game);
  return_and_pickup_flags(game);
}
