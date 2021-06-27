use std::time::Duration;

use airmash_protocol::PlaneType;
use smallvec::SmallVec;

use crate::component::*;
use crate::consts::hitcircles_for_plane;
use crate::event::EventBounce;
use crate::event::MissileTerrainCollision;
use crate::resource::{collision::*, LastFrame, ThisFrame};
use crate::AirmashWorld;

struct FrameId(usize);

pub fn generate_collision_lookups(game: &mut AirmashWorld) {
  generate_player_pos_db(game);
  generate_player_collide_db(game);
  generate_missile_collide_db(game);
}

pub fn check_collisions(game: &mut AirmashWorld) {
  let frame_id = {
    let frame = game.resources.entry().or_insert(FrameId(0));
    let frame_id = frame.0;
    frame.0 += 1;
    frame_id
  };
  let elapsed_time = {
    let this_frame = game.resources.read::<ThisFrame>().0;
    let last_frame = game.resources.read::<LastFrame>().0;
    this_frame - last_frame
  };

  // To more accurately emulate the original server we only do player-terrain
  // collisions every other frame.
  //
  // This is needed to make some well-liked things like the shortcut into
  // greenland work properly.
  if frame_id % 2 == 0 || elapsed_time > Duration::from_millis(30) {
    collide_player_terrain(game);
  }

  collide_missile_terrain(game);
}

fn generate_player_pos_db(game: &mut AirmashWorld) {
  let mut db = game.resources.write::<PlayerPosDb>();

  let query = game
    .world
    .query_mut::<(&Position, &Team)>()
    .with::<IsPlayer>();
  let mut entries = Vec::new();

  for (entity, (pos, team)) in query {
    entries.push(Entry {
      entity,
      pos: pos.0,
      radius: 0.0,
      layer: team.0,
    });
  }

  db.recreate(entries);
}

fn generate_player_collide_db(game: &mut AirmashWorld) {
  let mut db = game.resources.write::<PlayerCollideDb>();

  let query = game
    .world
    .query_mut::<(&Position, &Rotation, &PlaneType, &Team)>()
    .with::<IsPlayer>();
  let mut entries = Vec::new();

  for (entity, (pos, rot, plane, team)) in query {
    for hc in hitcircles_for_plane(*plane) {
      let offset = crate::util::rotate(hc.0, rot.0);

      entries.push(Entry {
        pos: pos.0 + offset,
        radius: hc.1,
        entity,
        layer: team.0,
      });
    }
  }

  db.recreate(entries);
}

fn generate_missile_collide_db(game: &mut AirmashWorld) {
  let mut db = game.resources.write::<MissileCollideDb>();

  let query = game
    .world
    .query_mut::<(&Position, &Team)>()
    .with::<IsMissile>();
  let mut entries = Vec::new();

  for (entity, (pos, team)) in query {
    entries.push(Entry {
      entity,
      pos: pos.0,
      radius: 0.0,
      layer: team.0,
    });
  }

  db.recreate(entries);
}

fn collide_player_terrain(game: &mut AirmashWorld) {
  use std::cmp::Ordering;

  let players = game.resources.read::<PlayerCollideDb>();
  let terrain = game.resources.read::<Terrain>();

  let mut collisions = Vec::new();
  players.query_all_pairs(&terrain.0, &mut collisions);

  // Only count the collision with the smallest distance
  collisions.sort_unstable_by(|a, b| match a.0.entity.id().cmp(&b.0.entity.id()) {
    Ordering::Equal => {
      let da: f32 = (a.0.pos - a.1.pos).norm_squared();
      let db: f32 = (a.0.pos - b.1.pos).norm_squared();

      da.partial_cmp(&db)
        .unwrap_or_else(|| match (da.is_nan(), db.is_nan()) {
          (true, true) => Ordering::Equal,
          (true, false) => Ordering::Greater,
          (false, true) => Ordering::Less,
          (false, false) => unreachable!(),
        })
    }
    x => x,
  });
  collisions.dedup_by_key(|entry| entry.0.entity);

  let mut events =  SmallVec::<[_; 32]>::new();
  for collision in collisions {
    let query = game
      .world
      .query_one_mut::<&mut Velocity>(collision.0.entity);
    let vel = match query {
      Ok(query) => query,
      Err(_) => continue,
    };

    let rel = collision.0.pos - collision.1.pos;
    let newvel = rel.normalize() * vel.norm().max(1.0);
    let old_vel = std::mem::replace(&mut vel.0, newvel);

    events.push(EventBounce {
      player: collision.0.entity,
      old_vel,
    });
  }

  drop(players);
  drop(terrain);

  for event in events {
    game.dispatch(event);
  }
}

fn collide_missile_terrain(game: &mut AirmashWorld) {
  use std::cmp::Ordering;

  let missiles = game.resources.read::<MissileCollideDb>();
  let terrain = game.resources.read::<Terrain>();

  let mut collisions = Vec::new();
  missiles.query_all_pairs(&terrain.0, &mut collisions);

  // Only count the collision with the smallest distance (so the missile only
  // explodes once)
  collisions.sort_unstable_by(|a, b| match a.0.entity.id().cmp(&b.0.entity.id()) {
    Ordering::Equal => {
      let da: f32 = (a.0.pos - a.1.pos).norm_squared();
      let db: f32 = (a.0.pos - b.1.pos).norm_squared();

      da.partial_cmp(&db)
        .unwrap_or_else(|| match (da.is_nan(), db.is_nan()) {
          (true, true) => Ordering::Equal,
          (true, false) => Ordering::Greater,
          (false, true) => Ordering::Less,
          (false, false) => unreachable!(),
        })
    }
    x => x,
  });
  collisions.dedup_by_key(|entry| entry.0.entity);

  let mut events = SmallVec::<[_; 32]>::new();
  for collision in collisions {
    events.push(MissileTerrainCollision {
      missile: collision.0.entity
    });
  }

  drop(missiles);
  drop(terrain);

  for event in events {
    game.dispatch(event);
    game.despawn(event.missile);
  }
}
