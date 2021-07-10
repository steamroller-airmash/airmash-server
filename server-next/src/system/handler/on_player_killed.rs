use crate::component::*;
use crate::event::PlayerKilled;
use crate::event::PlayerRespawn;
use crate::resource::{Config, GameConfig, TaskScheduler, ThisFrame};
use crate::AirmashGame;

#[handler]
fn launch_respawn_task(event: &PlayerKilled, game: &mut AirmashGame) {
  let tasks = game.resources.read::<TaskScheduler>();
  let config = game.resources.read::<Config>();
  let game_config = game.resources.read::<GameConfig>();
  let this_frame = game.resources.read::<ThisFrame>();

  let query = game
    .world
    .query_one_mut::<(&mut RespawnAllowed, &IsPlayer)>(event.player);
  let (can_respawn, ..) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  can_respawn.0 = false;

  if !game_config.default_respawn {
    return;
  }

  let event = *event;
  tasks.schedule(
    this_frame.0 + config.respawn_delay,
    move |game: &mut AirmashGame| {
      let query = game
        .world
        .query_one_mut::<(&mut RespawnAllowed, &IsSpectating, &IsPlayer)>(event.player);
      let (can_respawn, spectating, ..) = match query {
        Ok(query) => query,
        Err(_) => return,
      };

      can_respawn.0 = true;

      if !spectating.0 {
        game.dispatch(PlayerRespawn {
          player: event.player,
          alive: false,
        });
      }
    },
  );
}

#[handler]
fn set_dead_flag(event: &PlayerKilled, game: &mut AirmashGame) {
  let query = game
    .world
    .query_one_mut::<(&mut IsAlive, &IsPlayer)>(event.player);
  let (is_alive, ..) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  is_alive.0 = false;
}

#[handler]
fn send_player_killed_packets(event: &PlayerKilled, game: &mut AirmashGame) {
  use crate::protocol::server::PlayerKill;

  if Some(event.player) == event.killer {
    warn!("Player {:?} killed themselves?", event.player);
  }

  let view_radius = {
    let config = game.resources.read::<Config>();
    config.view_radius
  };

  let player_kill = {
    let query = game
      .world
      .query_one_mut::<(&Position, &IsPlayer)>(event.player);
    let (&pos, ..) = match query {
      Ok(query) => query,
      Err(_) => return,
    };
    PlayerKill {
      id: event.player.id() as _,
      killer: event.killer.map(|x| x.id() as _),
      pos: pos.0,
    }
  };

  game.send_to_visible(player_kill.pos, player_kill);

  // The killer may have left the game
  if let Some(killer) = event.killer {
    let query = game.world.query_one_mut::<(&Position, &IsPlayer)>(killer);
    let (&pos, ..) = match query {
      Ok(query) => query,
      Err(_) => return,
    };

    if (pos.0 - player_kill.pos).norm_squared() >= view_radius * view_radius {
      game.send_to(killer, player_kill);
    }
  };
}

#[handler(priority = crate::priority::MEDIUM)]
fn update_scores(event: &PlayerKilled, game: &mut AirmashGame) {
  if Some(event.player) == event.killer {
    return;
  }

  let mut pquery = match game
    .world
    .query_one::<(&mut DeathCount, &Score)>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  let transfer = match pquery.get() {
    Some((deaths, score)) => {
      deaths.0 += 1;

      ((score.0 + 4) / 5) as i32
    }
    None => return,
  };
  drop(pquery);
  let _ = game.update_score(event.player, -transfer);

  let killer = match event.killer {
    Some(killer) => killer,
    None => return,
  };

  let mut kquery = match game.world.query_one::<&mut KillCount>(killer) {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some(kills) = kquery.get() {
    kills.0 += 1;
  }

  drop(kquery);
  let _ = game.update_score(killer, transfer + 25);
}
