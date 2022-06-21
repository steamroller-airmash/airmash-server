use smallvec::SmallVec;

use crate::component::*;
use crate::event::PowerupExpire;
use crate::AirmashGame;

pub fn update(game: &mut AirmashGame) {
  expire_effects(game);
}

fn expire_effects(game: &mut AirmashGame) {
  let this_frame = game.this_frame();
  let query = game.world.query_mut::<&Effects>().with::<IsPlayer>();

  let mut events = SmallVec::<[_; 16]>::new();
  for (ent, effects) in query {
    match effects.expiry() {
      Some(expiry) if expiry <= this_frame => (),
      _ => continue,
    };

    events.push(PowerupExpire { player: ent });
  }

  for event in events {
    game.dispatch(event);
    game
      .world
      .get_mut::<Effects>(event.player)
      .unwrap()
      .clear_powerup();
    game.world.get_mut::<Powerup>(event.player).unwrap().data = None;
  }
}
