use std::time::Duration;

use airmash_protocol::Vector2;

use crate::event::PlayerKilled;
use crate::event::PlayerRespawn;
use crate::resource::{Config, GameConfig, TaskScheduler, ThisFrame};
use crate::AirmashGame;
use crate::{component::*, consts};

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

#[handler(priority = crate::priority::MEDIUM)]
fn update_upgrades(event: &PlayerKilled, game: &mut AirmashGame) {
  let (upgrades, _) = match game
    .world
    .query_one_mut::<(&mut Upgrades, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let offsets = rand::random::<u8>() & 0xF;

  upgrades.speed += (offsets >> 0) & 1;
  upgrades.defense += (offsets >> 1) & 1;
  upgrades.energy += (offsets >> 2) & 1;
  upgrades.missile += (offsets >> 3) & 1;

  upgrades.speed /= 2;
  upgrades.defense /= 2;
  upgrades.energy /= 2;
  upgrades.missile /= 2;
}

#[handler(priority = crate::priority::HIGH)]
fn drop_upgrade(event: &PlayerKilled, game: &mut AirmashGame) {
  let this_frame = game.this_frame();
  let config = game.resources.read::<Config>();
  let game_config = game.resources.read::<GameConfig>();

  // Do nothing if we aren't supposed to be spawning upgrades.
  if !game_config.spawn_upgrades {
    return;
  }

  let (upgrades, &pos, &vel, last_action, _) =
    match game
      .world
      .query_one_mut::<(&Upgrades, &Position, &Velocity, &LastActionTime, &IsPlayer)>(event.player)
    {
      Ok(query) => query,
      Err(_) => return,
    };

  let total_upgrades = upgrades.speed + upgrades.energy + upgrades.defense + upgrades.missile;
  if vel.0 == Vector2::zeros() && this_frame - last_action.0 > Duration::from_secs(10) {
    return;
  }

  let lifetime = config
    .mobs
    .upgrade
    .lifetime
    .unwrap_or(Duration::from_secs(60));
  let prob = rand::random::<f32>();

  drop(config);
  drop(game_config);

  if total_upgrades > 0 || prob < consts::UPGRADE_DROP_PROBABILITY {
    game.spawn_mob(MobType::Upgrade, pos.0, lifetime);
  }
}
