use smallvec::SmallVec;

use crate::component::IsMissile;
use crate::component::*;
use crate::event::{MissileDespawn, MissileDespawnType};
use crate::AirmashWorld;

pub fn update(game: &mut AirmashWorld) {
  despawn_missiles(game);
}

/// Missiles despawn after having travelled a configurable distance. Every frame
/// we need to check all missiles for those that should be despawned.
fn despawn_missiles(game: &mut AirmashWorld) {
  let mut query = game
    .world
    .query::<(&Position, &MissileTrajectory)>()
    .with::<IsMissile>();

  let mut events = SmallVec::<[MissileDespawn; 16]>::new();
  for (ent, (pos, traj)) in query.iter() {
    let dist = (pos.0 - traj.start).norm_squared();
    if dist > traj.maxdist * traj.maxdist {
      events.push(MissileDespawn {
        missile: ent,
        ty: MissileDespawnType::LifetimeEnded,
      });
    }
  }

  drop(query);

  for event in events {
    game.dispatch(event);
    game.despawn(event.missile);
  }
}
