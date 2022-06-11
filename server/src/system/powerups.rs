use smallvec::SmallVec;

use crate::component::*;
use crate::event::PowerupExpire;
use crate::resource::ThisFrame;
use crate::AirmashGame;

pub fn update(game: &mut AirmashGame) {
  expire_powerups(game);
}

fn expire_powerups(game: &mut AirmashGame) {
  let this_frame = game.resources.read::<ThisFrame>().0;
  let query = game.world.query_mut::<&Powerup>().with::<IsPlayer>();

  let mut events = SmallVec::<[_; 16]>::new();
  for (ent, powerup) in query {
    let powerup = match &powerup.data {
      Some(data) => data,
      None => continue,
    };

    if powerup.end_time > this_frame {
      continue;
    }

    events.push(PowerupExpire { player: ent });
  }

  for event in events {
    game.dispatch(event);
    game.world.get_mut::<Powerup>(event.player).unwrap().data = None;
  }
}
