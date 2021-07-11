use std::cmp::Ordering;

use hecs::Entity;
use itertools::Itertools;

use crate::component::*;
use crate::AirmashGame;

pub enum SpectateTarget {
  Next,
  Prev,
  Force,
  Target(u16),
}

fn wrapped_compare(ent: Entity) -> impl Fn(&Entity, &Entity) -> Ordering {
  move |a, b| {
    let aid = a.id().wrapping_sub(ent.id());
    let bid = b.id().wrapping_sub(ent.id());

    aid.cmp(&bid)
  }
}

pub fn spectate_target(
  player: Entity,
  target: Option<Entity>,
  spec: SpectateTarget,
  game: &AirmashGame,
) -> Option<Entity> {
  let minmax = game
    .world
    .query::<&IsAlive>()
    .with::<IsPlayer>()
    .iter()
    .filter(|&(_, alive)| alive.0)
    .filter(|&(e, _)| e != player)
    .filter(|&(e, _)| Some(e) != target)
    .map(|(e, _)| e)
    .minmax_by(wrapped_compare(target.unwrap_or(player)));

  match minmax.into_option() {
    Some((min, max)) => {
      let default = target.unwrap_or(min);

      Some(match spec {
        SpectateTarget::Force => min,
        SpectateTarget::Next => min,
        SpectateTarget::Prev => max,
        SpectateTarget::Target(id) => match game.find_entity_by_id(id) {
          Some(entity) => match game.world.query_one::<&IsAlive>(entity) {
            Ok(query) => match query.with::<IsPlayer>().get() {
              Some(alive) if alive.0 => entity,
              _ => default,
            },
            _ => default,
          },
          None => match target {
            Some(tgt) => tgt,
            None => default,
          },
        },
      })
    }
    None => target,
  }
}
