use crate::event::GameStartEvent;
use crate::{component::IsFlag, event::FlagEvent, resource::*};
use airmash::event::*;
use airmash::AirmashGame;
use airmash::{component::*, resource::GameConfig};
use smallvec::SmallVec;

#[handler]
fn respawn_all_players(_: &GameStartEvent, game: &mut AirmashGame) {
  let mut events = Vec::new();
  let query = game
    .world
    .query_mut::<(&IsSpectating, &IsAlive)>()
    .with::<IsPlayer>();
  for (player, (spec, alive)) in query {
    if spec.0 {
      continue;
    }

    events.push(PlayerRespawn {
      player,
      alive: alive.0,
    });
  }

  game.dispatch_many(events);
}

#[handler(priority = airmash::priority::MEDIUM)]
fn reset_damage(_: &GameStartEvent, game: &mut AirmashGame) {
  // Reset the scores
  *game.resources.write::<GameScores>() = GameScores::default();

  // Allow players to deal damage to each other.
  game.resources.write::<GameConfig>().allow_damage = true;

  // Allow players to pick up flags again
  game.resources.write::<GameActive>().0 = true;
}

#[handler]
fn reset_flags(_: &GameStartEvent, game: &mut AirmashGame) {
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
