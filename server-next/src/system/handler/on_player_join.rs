use airmash_protocol::GameType;
use airmash_protocol::{FlagCode, PlaneType, PlayerStatus};
use bstr::BString;

use crate::component::*;
use crate::event::PlayerJoin;
use crate::resource::GameRoom;
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
      &PlaneType,
      &Team,
      &Position,
      &Rotation,
      &FlagCode,
      &Upgrades,
      &Powerup,
    )>()
    .with::<IsPlayer>();
  let players = query
    .into_iter()
    .map(
      |(ent, (alive, level, name, plane, team, pos, rot, flag, upgrades, powerup))| LoginPlayer {
        id: ent.id() as u16,
        status: PlayerStatus::from(*alive),
        level: level.0,
        name: name.0.clone(),
        ty: *plane,
        team: team.0,
        pos: pos.0,
        rot: rot.0,
        flag: *flag,
        upgrades: crate::util::get_server_upgrades(upgrades, powerup),
      },
    )
    .collect::<Vec<_>>();

  let clock = crate::util::get_current_clock(game);
  let game_ty = *game.resources.read::<GameType>();
  let room = game.resources.read::<GameRoom>().clone();

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
    &PlaneType,
    &Team,
    &Position,
    &Rotation,
    &FlagCode,
    &Upgrades,
    &Powerup,
  )>(event.player)
  {
    Ok(query) => query.with::<IsPlayer>(),
    Err(_) => return,
  };

  if let Some((alive, name, plane, team, pos, rot, flag, upgrades, powerup)) = query.get() {
    let packet = PlayerNew {
      id: event.player.id() as _,
      status: PlayerStatus::from(*alive),
      name: name.0.clone(),
      ty: *plane,
      team: team.0,
      pos: pos.0,
      rot: rot.0,
      flag: *flag,
      upgrades: crate::util::get_server_upgrades(upgrades, powerup),
    };

    game.send_to_others(event.player, packet);
  }
}

#[handler]
fn update_server_stats(_: &PlayerJoin, _: &mut AirmashGame) {
  use crate::network::NUM_PLAYERS;

  NUM_PLAYERS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}
