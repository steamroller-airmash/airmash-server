use airmash_protocol::{FlagCode, GameType, PlayerStatus};
use bstr::BString;

use crate::component::*;
use crate::config::PlanePrototypeRef;
use crate::event::PlayerJoin;
use crate::resource::{RegionName, ServerStats};
use crate::world::AirmashGame;

#[handler(priority = crate::priority::LOGIN)]
fn send_login_packet(event: &PlayerJoin, game: &mut AirmashGame) {
  use crate::protocol::server::{Login, LoginPlayer};

  let mut query = game
    .world
    .query::<(
      &IsAlive,
      &Level,
      &Name,
      &PlanePrototypeRef,
      &Team,
      &Position,
      &Rotation,
      &FlagCode,
      &Upgrades,
      &Effects,
    )>()
    .with::<IsPlayer>();
  let players = query
    .into_iter()
    .map(
      |(ent, (alive, level, name, plane, team, pos, rot, flag, upgrades, effects))| LoginPlayer {
        id: ent.id() as u16,
        status: PlayerStatus::from(*alive),
        level: level.0,
        name: name.0.clone(),
        ty: plane.server_type,
        team: team.0,
        pos: pos.0,
        rot: rot.0,
        flag: *flag,
        upgrades: crate::util::get_server_upgrades(upgrades, effects),
      },
    )
    .collect::<Vec<_>>();

  let clock = crate::util::get_current_clock(game);
  let game_ty = *game.resources.read::<GameType>();
  let room = game.resources.read::<RegionName>().clone();

  let mut query = match game.world.query_one::<(&Team, &Session)>(event.player) {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  debug!("Sending login packet to player id {:?}", event.player);

  if let Some((team, session)) = query.get() {
    let packet = Login {
      success: true,
      id: event.player.id() as u16,
      team: team.0,
      clock,
      token: BString::from(format!("{}", session.0)),
      ty: game_ty,
      room: room.0.into(),
      players,
    };

    game.send_to(event.player, packet);
  } else {
    warn!("Player {:?} missing required components", event.player);
  }
}

#[handler]
fn send_level_packet(event: &PlayerJoin, game: &mut AirmashGame) {
  use crate::protocol::server::PlayerLevel;

  let mut query = match game.world.query_one::<&Level>(event.player) {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some(level) = query.get() {
    game.send_to_others(
      event.player,
      PlayerLevel {
        id: event.player.id() as _,
        ty: airmash_protocol::PlayerLevelType::Login,
        level: level.0,
      },
    );
  }
}

#[handler]
fn send_player_new(event: &PlayerJoin, game: &mut AirmashGame) {
  use crate::protocol::server::PlayerNew;

  let mut query = match game.world.query_one::<(
    &IsAlive,
    &Name,
    &PlanePrototypeRef,
    &Team,
    &Position,
    &Rotation,
    &FlagCode,
    &Upgrades,
    &Effects,
  )>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some((alive, name, plane, team, pos, rot, flag, upgrades, effects)) = query.get() {
    let packet = PlayerNew {
      id: event.player.id() as _,
      status: PlayerStatus::from(*alive),
      name: name.0.clone(),
      ty: plane.server_type,
      team: team.0,
      pos: pos.0,
      rot: rot.0,
      flag: *flag,
      upgrades: crate::util::get_server_upgrades(upgrades, effects),
    };

    game.send_to_others(event.player, packet);
  }
}

#[handler]
fn send_score_update(event: &PlayerJoin, game: &mut AirmashGame) {
  use crate::protocol::server::ScoreUpdate;

  let (score, earnings, deaths, kills, upgrades) =
    match game
      .world
      .query_one_mut::<(&Score, &Earnings, &DeathCount, &KillCount, &Upgrades)>(event.player)
    {
      Ok(query) => query,
      Err(_) => return,
    };

  let packet = ScoreUpdate {
    id: event.player.id() as _,
    score: score.0,
    earnings: earnings.0,
    total_deaths: deaths.0,
    total_kills: kills.0,
    upgrades: upgrades.unused,
  };
  game.send_to(event.player, packet);
}

#[handler]
fn update_server_stats(_: &PlayerJoin, game: &mut AirmashGame) {
  use crate::network::NUM_PLAYERS;

  let mut stats = game.resources.write::<ServerStats>();

  stats.num_players += 1;
  NUM_PLAYERS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

#[handler(priority = crate::priority::CLEANUP)]
fn dispatch_player_spawn(event: &PlayerJoin, game: &mut AirmashGame) {
  use crate::event::PlayerSpawn;

  let alive = match game.world.query_one_mut::<&IsAlive>(event.player) {
    Ok(alive) => alive.0,
    Err(_) => return,
  };

  if alive {
    game.dispatch(PlayerSpawn {
      player: event.player,
    });
  }
}
