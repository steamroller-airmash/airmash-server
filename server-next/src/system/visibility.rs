use crate::component::*;
use crate::event::EntitySpawn;
use crate::event::EventHorizon;
use crate::event::PlayerFire;
use crate::resource::collision as c;
use crate::resource::collision::LayerSpec;
use crate::resource::Config;
use crate::AirmashWorld;

use hecs::Entity;
use std::collections::HashSet;
use std::iter::FromIterator;

def_wrappers! {
  type FrameId = u64;

  /// All entities visible to the current one. This is used to determine when
  /// entities move beyond a player's horizon.
  ##[nocopy]
  type VisibleEntities = HashSet<Entity>;
}

pub fn generate_horizon_events(game: &mut AirmashWorld) {
  let frame = {
    let frame = game.resources.entry::<FrameId>().or_insert(FrameId(0));
    frame.0 += 1;
    frame.0
  };

  let missile_db = game.resources.read::<c::MissileCollideDb>();
  let player_db = game.resources.read::<c::PlayerPosDb>();
  let config = game.resources.read::<Config>();

  let query = game
    .world
    .query_mut::<(&Position, &FrameId, &mut VisibleEntities)>()
    .with::<IsPlayer>();

  let mut vis_missiles = Vec::new();
  let mut vis_players = Vec::new();
  let mut actions = Vec::<EventHorizon>::new();

  for (ent, (pos, spawn_frame, visible)) in query {
    // Don't send any updates for players who just joined this frame
    if spawn_frame.0 == frame {
      continue;
    }

    missile_db.query(
      pos.0,
      config.view_radius,
      LayerSpec::None,
      &mut vis_missiles,
    );
    player_db.query(pos.0, config.view_radius, LayerSpec::None, &mut vis_players);

    let new_vis = HashSet::from_iter(
      vis_missiles
        .drain(..)
        .chain(vis_players.drain(..))
        .filter(|&x| x != ent),
    );

    let old_vis = std::mem::replace(&mut visible.0, new_vis);

    for lost in old_vis.difference(&visible.0).copied() {
      actions.push(EventHorizon {
        player: ent,
        entity: lost,
        in_horizon: false,
      });
    }

    for found in visible.difference(&old_vis).copied() {
      actions.push(EventHorizon {
        player: ent,
        entity: found,
        in_horizon: true,
      });
    }
  }

  drop(config);
  drop(missile_db);
  drop(player_db);

  for action in actions {
    game.dispatch(action);
  }
}

#[handler]
fn record_entity_spawn_frame(event: &EntitySpawn, game: &mut AirmashWorld) {
  let frame = game.resources.get::<FrameId>().map(|x| x.0).unwrap_or(0);
  let _ = game.world.insert(
    event.entity,
    (FrameId(frame), VisibleEntities(HashSet::default())),
  );
}

/// New missiles need to be added to the respective visible entity set of
/// players within range.
///
/// That way we can properly send horizon updates if they travel outside of a
/// player's horizon within a single frame.
#[handler]
fn record_new_missiles(event: &PlayerFire, game: &mut AirmashWorld) {
  let ppos = match game
    .world
    .query_one_mut::<(&Position, &IsPlayer)>(event.player)
  {
    Ok((pos, _)) => pos.0,
    Err(_) => return,
  };

  let config = game.resources.read::<Config>();
  let view2 = config.view_radius * config.view_radius;

  let query = game
    .world
    .query_mut::<(&Position, &mut VisibleEntities)>()
    .with::<IsPlayer>();

  for (_, (pos, visible)) in query {
    if (pos.0 - ppos).norm_squared() <= view2 {
      visible.extend(event.missiles.iter().copied());
    }
  }
}
