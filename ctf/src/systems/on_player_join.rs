use std::cmp::Ordering;

use airmash::component::*;
use airmash::event::PlayerJoin;
use airmash::{AirmashGame, Vector2};

use crate::component::*;
use crate::config;
use crate::resource::{CTFGameStats, GameScores};

#[handler(priority = airmash::priority::PRE_LOGIN)]
fn setup_team_and_pos(event: &PlayerJoin, game: &mut AirmashGame) {
  let stats = game.resources.read::<CTFGameStats>();
  let score = game.resources.read::<GameScores>();

  let team = match stats.red_players.cmp(&stats.blue_players) {
    Ordering::Less => config::RED_TEAM,
    Ordering::Greater => config::BLUE_TEAM,
    Ordering::Equal => match score.redteam.cmp(&score.blueteam) {
      Ordering::Less => config::RED_TEAM,
      Ordering::Greater => config::BLUE_TEAM,
      Ordering::Equal => match rand::random() {
        true => config::RED_TEAM,
        false => config::BLUE_TEAM,
      },
    },
  };

  let offset = Vector2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
  let respawn = config::team_respawn_pos(team) + 400.0 * offset;

  let _ = game
    .world
    .insert(event.player, (Team(team), Position(respawn)));
}

#[handler]
fn send_flag_position_on_join(event: &PlayerJoin, game: &mut AirmashGame) {
  use airmash::protocol::server::GameFlag;
  use airmash::protocol::FlagUpdateType;

  let scores = game.resources.read::<GameScores>();
  let mut query = game
    .world
    .query::<(&Position, &Team, &FlagCarrier)>()
    .with::<IsFlag>();

  for (_, (pos, team, carrier)) in query.iter() {
    let ty = match carrier.0 {
      Some(_) => FlagUpdateType::Carrier,
      None => FlagUpdateType::Position,
    };

    let packet = GameFlag {
      ty,
      pos: pos.into(),
      flag: team.0 as _,
      id: carrier.0.map(|x| x.id() as _),
      blueteam: scores.blueteam,
      redteam: scores.redteam,
    };
    game.send_to(event.player, packet);
  }
}

#[handler]
fn update_player_count(event: &PlayerJoin, game: &mut AirmashGame) {
  let mut counts = game.resources.write::<CTFGameStats>();

  let team = match game.world.get::<Team>(event.player) {
    Ok(team) => team,
    Err(e) => {
      warn!(
        "Newly joined player {:?} missing team component: {}",
        event.player, e
      );
      return;
    }
  };

  match team.0 {
    config::BLUE_TEAM => counts.blue_players += 1,
    config::RED_TEAM => counts.red_players += 1,
    x => warn!(
      "Newly joined player {:?} had unexpected team: {}",
      event.player, x
    ),
  }
}
