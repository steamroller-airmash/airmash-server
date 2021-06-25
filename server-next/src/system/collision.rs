use airmash_protocol::{PlaneType, Vector2};

use crate::component::{IsMissile, IsPlayer, Position, Rotation, Team};
use crate::consts::hitcircles_for_plane;
use crate::resource::collision::*;
use crate::AirmashWorld;

pub fn update(game: &mut AirmashWorld) {
  generate_player_pos_db(game);
  generate_player_collide_db(game);
  generate_missile_collide_db(game);
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

fn rotate(v: Vector2<f32>, angle: f32) -> Vector2<f32> {
  let (sin, cos) = angle.sin_cos();

  Vector2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

fn generate_player_collide_db(game: &mut AirmashWorld) {
  let mut db = game.resources.write::<PlayerCollideDb>();

  let query = game
    .world
    .query_mut::<(&Position, &Rotation, &PlaneType, &Team)>()
    .with::<IsMissile>();
  let mut entries = Vec::new();

  for (entity, (pos, rot, plane, team)) in query {
    for hc in hitcircles_for_plane(*plane) {
      let offset = rotate(hc.0, rot.0);

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
