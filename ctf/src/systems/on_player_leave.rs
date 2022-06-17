use airmash::component::*;
use airmash::event::PlayerLeave;
use airmash::AirmashGame;
use smallvec::SmallVec;

use crate::component::{FlagCarrier, IsFlag};
use crate::config;
use crate::event::FlagEvent;
use crate::resource::CTFGameStats;

#[handler]
fn drop_flag(event: &PlayerLeave, game: &mut AirmashGame) {
  let query = game.world.query_mut::<&FlagCarrier>().with::<IsFlag>();

  let mut events = SmallVec::<[_; 2]>::new();
  for (flag, carrier) in query {
    if carrier.0 != Some(event.player) {
      continue;
    }

    events.push(FlagEvent {
      ty: crate::event::FlagEventType::Drop,
      player: Some(event.player),
      flag,
    })
  }
}

#[handler]
fn update_player_count(event: &PlayerLeave, game: &mut AirmashGame) {
  let mut counts = game.resources.write::<CTFGameStats>();

  let team = match game.world.get::<Team>(event.player) {
    Ok(team) => team,
    Err(_) => return,
  };

  match team.0 {
    config::BLUE_TEAM => counts.blue_players -= 1,
    config::RED_TEAM => counts.red_players -= 1,
    _ => (),
  }
}
