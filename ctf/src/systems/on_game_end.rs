use std::time::Duration;

use crate::{
  component::IsFlag,
  config::GAME_WIN_BOUNTY_BASE,
  event::{FlagEvent, GameStartEvent},
  resource::GameActive,
};
use crate::{event::GameEndEvent, shuffle::ShuffleType};
use airmash::resource::GameConfig;
use airmash::resource::{ServerStats, TaskScheduler};
use airmash::AirmashGame;
use smallvec::SmallVec;

fn display_message(game: &mut AirmashGame, msg: &str, duration: u32) {
  use airmash::protocol::server::ServerMessage;

  game.send_to_all(ServerMessage {
    ty: airmash::protocol::ServerMessageType::TimeToGameStart,
    duration: duration * 1000,
    text: msg.into(),
  });
}

fn shuffle_players(game: &mut AirmashGame) {
  use airmash::component::Team;
  use airmash::protocol::server::{PlayerReteam, PlayerReteamPlayer};

  let shuffle = crate::shuffle::shuffle(&mut *game, ShuffleType::AlternatingScore);
  let mut players = Vec::with_capacity(shuffle.len());

  for change in shuffle {
    let _ = game.world.insert_one(change.player, Team(change.team));

    players.push(PlayerReteamPlayer {
      id: change.player.id() as _,
      team: change.team,
    });
  }

  let packet = PlayerReteam { players };

  game.send_to_all(packet);
}

#[handler]
fn schedule_tasks(_: &GameEndEvent, game: &mut AirmashGame) {
  let scheduler = game.resources.read::<TaskScheduler>().clone();

  unsafe {
    scheduler.spawn(move |mut game| async move {
      display_message(&mut *game, "New game starting in 1 minute", 12);
      game.sleep_for(Duration::from_secs(30)).await;

      // Shuffle all players
      shuffle_players(&mut game);

      // Display countdown
      display_message(&mut game, "Game starting in 30 seconds", 7);
      game.sleep_for(Duration::from_secs(20)).await;
      display_message(&mut game, "Game starting in 10 seconds", 7);
      game.sleep_for(Duration::from_secs(5)).await;
      display_message(&mut game, "Game starting in 5 seconds", 2);
      game.sleep_for(Duration::from_secs(1)).await;
      display_message(&mut game, "Game starting in 4 seconds", 2);
      game.sleep_for(Duration::from_secs(1)).await;
      display_message(&mut game, "Game starting in 3 seconds", 2);
      game.sleep_for(Duration::from_secs(1)).await;
      display_message(&mut game, "Game starting in 2 seconds", 2);
      game.sleep_for(Duration::from_secs(1)).await;
      display_message(&mut game, "Game starting in 1 second", 2);
      game.sleep_for(Duration::from_secs(1)).await;
      display_message(&mut game, "Game starting!", 3);

      // Emit game start event
      game.dispatch(GameStartEvent);
    });
  }
}

#[handler]
fn award_team_bounty(event: &GameEndEvent, game: &mut AirmashGame) {
  use airmash::component::*;

  let mut players = Vec::new();
  let query = game.world.query_mut::<&Team>().with::<IsPlayer>();

  for (player, team) in query {
    if team.0 != event.winning_team {
      continue;
    }

    players.push(player);
  }

  for player in players {
    let _ = game.update_score(player, 1000);
  }
}

#[handler]
fn send_game_end_packet(event: &GameEndEvent, game: &mut AirmashGame) {
  use airmash::protocol::server::ServerCustom;

  let stats = game.resources.read::<ServerStats>();

  let text = format!(
    "{{\"w\":{},\"b\":{},\"t\":{}}}",
    event.winning_team,
    stats.num_players.min(10) * GAME_WIN_BOUNTY_BASE,
    13 // display time in seconds
  );
  drop(stats);

  let packet = ServerCustom {
    ty: airmash::protocol::ServerCustomType::CTFWin,
    data: text.into(),
  };

  game.send_to_all(packet);
}

#[handler]
fn disable_damage(_: &GameEndEvent, game: &mut AirmashGame) {
  // Prevent players from being allowed to deal damage to each other.
  game.resources.write::<GameConfig>().allow_damage = false;

  // Prevent players from being allowed to pickup flags
  game.resources.write::<GameActive>().0 = false;
}

#[handler]
fn reset_flags(_: &GameEndEvent, game: &mut AirmashGame) {
  let mut events = SmallVec::<[_; 2]>::new();
  for (flag, ()) in game.world.query_mut::<()>().with::<IsFlag>() {
    events.push(FlagEvent {
      flag,
      player: None,
      ty: crate::event::FlagEventType::Return,
    });
  }

  game.dispatch_many(events);
}
