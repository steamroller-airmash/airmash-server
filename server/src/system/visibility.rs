use fxhash::FxHashSet as HashSet;
use hecs::Entity;

use crate::component::*;
use crate::event::{EntitySpawn, EventHorizon, MobSpawn, PlayerFire};
use crate::resource::collision::LayerSpec;
use crate::resource::{collision as c, GameConfig};
use crate::AirmashGame;

def_wrappers! {
  type FrameId = u64;

  /// All entities visible to the current one. This is used to determine when
  /// entities move beyond a player's horizon.
  ##[nocopy]
  type VisibleEntities = HashSet<Entity>;
}

pub fn generate_horizon_events(game: &mut AirmashGame) {
  let frame = {
    let frame = game.resources.entry::<FrameId>().or_insert(FrameId(0));
    frame.0 += 1;
    frame.0
  };

  let missile_db = game.resources.read::<c::MissileCollideDb>();
  let player_db = game.resources.read::<c::PlayerPosDb>();
  let mob_db = game.resources.read::<c::MobCollideDb>();
  let config = game.resources.read::<GameConfig>();

  let query = game
    .world
    .query_mut::<(&Position, &FrameId, &mut VisibleEntities)>()
    .with::<IsPlayer>();

  let mut vis_missiles = Vec::new();
  let mut vis_players = Vec::new();
  let mut vis_mobs = Vec::new();
  let mut actions = Vec::<EventHorizon>::new();
  let mut new_vis = HashSet::default();

  for (ent, (pos, spawn_frame, visible)) in query {
    // Don't send any updates for players who just joined this frame
    if spawn_frame.0 == frame {
      continue;
    }

    missile_db.query_pos(
      pos.0,
      config.view_radius,
      LayerSpec::None,
      &mut vis_missiles,
    );
    player_db.query_pos(pos.0, config.view_radius, LayerSpec::None, &mut vis_players);
    mob_db.query_pos(pos.0, config.view_radius, LayerSpec::None, &mut vis_mobs);

    new_vis.clear();
    new_vis.extend(
      vis_players
        .drain(..)
        .filter(|&x| x != ent)
        .chain(vis_missiles.drain(..))
        .chain(vis_mobs.drain(..)),
    );

    std::mem::swap(&mut visible.0, &mut new_vis);
    let old_vis = &new_vis;

    for lost in old_vis.difference(&visible.0).copied() {
      actions.push(EventHorizon {
        player: ent,
        entity: lost,
        in_horizon: false,
      });
    }

    for found in visible.difference(old_vis).copied() {
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
  drop(mob_db);

  for action in actions {
    game.dispatch(action);
  }
}

#[handler]
fn record_entity_spawn_frame(event: &EntitySpawn, game: &mut AirmashGame) {
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
fn record_new_missiles(event: &PlayerFire, game: &mut AirmashGame) {
  let ppos = match game
    .world
    .query_one_mut::<(&Position, &IsPlayer)>(event.player)
  {
    Ok((pos, _)) => pos.0,
    Err(_) => return,
  };

  let config = game.resources.read::<GameConfig>();
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

#[handler]
fn record_new_mobs(event: &MobSpawn, game: &mut AirmashGame) {
  let mpos = match game.world.query_one_mut::<(&Position, &IsMob)>(event.mob) {
    Ok((pos, _)) => pos.0,
    Err(_) => return,
  };

  let config = game.resources.read::<GameConfig>();
  let view2 = config.view_radius * config.view_radius;

  let query = game
    .world
    .query_mut::<(&Position, &mut VisibleEntities)>()
    .with::<IsPlayer>();

  for (_, (pos, visible)) in query {
    if (pos.0 - mpos).norm_squared() <= view2 {
      visible.insert(event.mob);
    }
  }
}
