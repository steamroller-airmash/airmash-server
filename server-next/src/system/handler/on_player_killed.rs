use crate::component::*;
use crate::event::PlayerKilled;
use crate::event::PlayerRespawn;
use crate::resource::{Config, GameConfig, TaskScheduler, ThisFrame};
use crate::AirmashWorld;

#[handler]
fn launch_respawn_task(event: &PlayerKilled, game: &mut AirmashWorld) {
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
    move |game: &mut AirmashWorld| {
      let query = game
        .world
        .query_one_mut::<(&mut RespawnAllowed, &IsPlayer)>(event.player);
      let (can_respawn, ..) = match query {
        Ok(query) => query,
        Err(_) => return,
      };

      can_respawn.0 = true;

      game.dispatch(PlayerRespawn {
        player: event.player,
      });
    },
  );
}

#[handler]
fn set_dead_flag(event: &PlayerKilled, game: &mut AirmashWorld) {
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
fn send_player_killed_packets(event: &PlayerKilled, game: &mut AirmashWorld) {
  use crate::protocol::server::{PlayerKill, ScoreUpdate};

  if event.player == event.killer {
    warn!("Player {:?} killed themselves?", event.player);
  }

  let view_radius = {
    let config = game.resources.read::<Config>();
    config.view_radius
  };

  let (player_su, player_kill) = {
    let query = game.world.query_one_mut::<(
      &Position,
      &Score,
      &KillCount,
      &DeathCount,
      &Earnings,
      &Upgrades,
      &IsPlayer,
    )>(event.player);
    let (&pos, &score, &kills, &deaths, &earnings, &upgrades, ..) = match query {
      Ok(query) => query,
      Err(_) => return,
    };
    let su = ScoreUpdate {
      id: event.player.id() as _,
      score: score.0,
      earnings: earnings.0,
      upgrades: upgrades.unused,
      total_deaths: deaths.0,
      total_kills: kills.0,
    };

    let kill = PlayerKill {
      id: event.player.id() as _,
      killer: Some(event.killer.id() as _),
      pos: pos.0,
    };

    (su, kill)
  };

  let (killer_su, killer_pos) = {
    let query = game.world.query_one_mut::<(
      &Position,
      &Score,
      &KillCount,
      &DeathCount,
      &Earnings,
      &Upgrades,
      &IsPlayer,
    )>(event.killer);
    let (&pos, &score, &kills, &deaths, &earnings, &upgrades, ..) = match query {
      Ok(query) => query,
      Err(_) => return,
    };
    let su = ScoreUpdate {
      id: event.killer.id() as _,
      score: score.0,
      earnings: earnings.0,
      upgrades: upgrades.unused,
      total_deaths: deaths.0,
      total_kills: kills.0,
    };

    (su, pos.0)
  };

  game.send_to_visible(player_kill.pos, player_kill);
  
  if (killer_pos - player_kill.pos).norm_squared() >= view_radius * view_radius {
    game.send_to(event.killer, player_kill);
  }

  game.send_to(event.player, player_su);
  game.send_to(event.killer, killer_su);
}

#[handler(priority = crate::priority::MEDIUM)]
fn update_scores(event: &PlayerKilled, game: &mut AirmashWorld) {
  if event.player == event.killer {
    return;
  }

  let mut pquery = match game
    .world
    .query_one::<(&mut DeathCount, &mut Score)>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  let transfer = match pquery.get() {
    Some((deaths, score)) => {
      let transfer = (score.0 + 4) / 5;

      score.0 -= transfer;
      deaths.0 += 1;

      transfer
    }
    None => return,
  };
  drop(pquery);

  let mut kquery = match game
    .world
    .query_one::<(&mut KillCount, &mut Score, &mut Earnings)>(event.killer)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some((kills, score, earnings)) = kquery.get() {
    kills.0 += 1;
    score.0 += transfer + 25;
    earnings.0 += transfer + 25;
  }
}
