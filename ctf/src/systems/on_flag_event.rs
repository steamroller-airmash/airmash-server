use crate::component::*;
use crate::config;
use crate::event::*;
use crate::resource::*;
use airmash::protocol::server::{GameFlag, ServerMessage};
use airmash::protocol::FlagUpdateType;
use airmash::AirmashGame;
use airmash::{component::*, resource::ServerStats};

#[handler]
fn check_game_win(event: &FlagEvent, game: &mut AirmashGame) {
  let scores = game.resources.read::<GameScores>();

  if event.ty != FlagEventType::Capture {
    return;
  }

  if scores.redteam < 3 && scores.blueteam < 3 {
    return;
  }

  let winner = match () {
    _ if scores.redteam >= 3 => config::RED_TEAM,
    _ => config::BLUE_TEAM,
  };

  drop(scores);

  game.dispatch(GameEndEvent {
    winning_team: winner,
  });
}

#[handler]
fn display_banner(event: &FlagEvent, game: &mut AirmashGame) {
  let verb = match event.ty {
    FlagEventType::Return => "Returned",
    FlagEventType::PickUp => "Taken",
    FlagEventType::Capture => "Captured",
    FlagEventType::Drop => return,
  };

  // If this event happens on it's own (end of game or system event) then don't
  // display a message.
  if event.player.is_none() {
    return;
  }

  let team = match game.world.query_one_mut::<(&Team, &IsFlag)>(event.flag) {
    Ok(query) => *query.0,
    Err(_) => return,
  };

  let name = match game.world.get_mut::<Name>(event.player.unwrap()) {
    Ok(name) => name.0.clone(),
    Err(_) => return,
  };

  let message = format!(
    "<span class=\"info inline\"><span class=\"{}\"></span></span>{} by {}",
    config::flag_message_team(team.0),
    verb,
    htmlescape::encode_minimal(&name.to_string())
  );

  let packet = ServerMessage {
    ty: airmash::protocol::ServerMessageType::Flag,
    duration: 3000,
    text: message.into(),
  };

  game.send_to_all(packet);
}

#[handler]
fn update_flag(event: &FlagEvent, game: &mut AirmashGame) {
  let this_frame = game.this_frame();
  let (pos, team, carrier, last_drop, _) = match game.world.query_one_mut::<(
    &mut Position,
    &Team,
    &mut FlagCarrier,
    &mut LastDrop,
    &IsFlag,
  )>(event.flag)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  match event.ty {
    FlagEventType::Capture | FlagEventType::Return => {
      pos.0 = config::flag_home_pos(team.0);
      carrier.0 = None;
    }
    FlagEventType::Drop => {
      carrier.0 = None;
      *last_drop = LastDrop {
        player: event.player,
        time: this_frame,
      };
    }
    FlagEventType::PickUp => {
      carrier.0 = event.player;
    }
  }
}

#[handler]
fn send_game_flag(event: &FlagEvent, game: &mut AirmashGame) {
  let ty = match event.ty {
    FlagEventType::PickUp => FlagUpdateType::Carrier,
    _ => FlagUpdateType::Position,
  };

  let (&pos, &team, _) = match game
    .world
    .query_one_mut::<(&Position, &Team, &IsFlag)>(event.flag)
  {
    Ok(query) => query,
    Err(e) => {
      warn!("Unable to read flag: {}", e);
      return;
    }
  };

  let scores = game.resources.read::<GameScores>();

  let carrier = match event.ty {
    FlagEventType::Capture => None,
    _ => event.player.map(|x| x.id() as _),
  };

  game.send_to_all(GameFlag {
    ty,
    flag: team.0 as _,
    pos: pos.0,
    id: carrier,
    blueteam: scores.blueteam,
    redteam: scores.redteam,
  });
}

#[handler(priority = airmash::priority::HIGH)]
fn update_game_scores(event: &FlagEvent, game: &mut AirmashGame) {
  if event.ty != FlagEventType::Capture {
    return;
  }

  let mut scores = game.resources.write::<GameScores>();

  let (team, _) = match game.world.query_one_mut::<(&Team, &IsFlag)>(event.flag) {
    Ok(query) => query,
    Err(_) => return,
  };

  if team.0 == config::BLUE_TEAM {
    scores.blueteam += 1;
  } else {
    scores.redteam += 1;
  }
}

#[handler(priority = airmash::priority::HIGH)]
fn update_player(event: &FlagEvent, game: &mut AirmashGame) {
  if event.ty != FlagEventType::Capture {
    return;
  }

  let player = match event.player {
    Some(player) => player,
    None => return,
  };

  let num_players = game.resources.read::<ServerStats>().num_players;

  let (captures, _) = match game
    .world
    .query_one_mut::<(&mut Captures, &IsPlayer)>(player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  captures.0 += 1;

  let score_increase = config::FLAG_CAP_BOUNTY_BASE * num_players.min(10);
  let _ = game.update_score(player, score_increase as i32);
}

#[handler]
fn update_player_keystate(event: &FlagEvent, game: &mut AirmashGame) {
  let player = match event.player {
    Some(player) => player,
    None => return,
  };

  let (keystate, _) = match game
    .world
    .query_one_mut::<(&mut KeyState, &IsPlayer)>(player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  match event.ty {
    FlagEventType::Capture => keystate.flagspeed = false,
    FlagEventType::Drop => keystate.flagspeed = false,
    FlagEventType::PickUp => keystate.flagspeed = true,
    FlagEventType::Return => return,
  };

  game.force_update(player);
}
