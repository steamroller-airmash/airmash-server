use smallvec::SmallVec;

use crate::component::{IsMissile, *};
use crate::event::{MissileDespawn, MissileDespawnType, MobDespawn, MobDespawnType};
use crate::util::NalgebraExt;
use crate::AirmashGame;

pub fn update(game: &mut AirmashGame) {
  despawn_missiles(game);
  despawn_mobs(game);
}

/// Missiles despawn after having travelled a configurable distance. Every frame
/// we need to check all missiles for those that should be despawned.
fn despawn_missiles(game: &mut AirmashGame) {
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

/// Mobs despawn after a configurable amount of time.
fn despawn_mobs(game: &mut AirmashGame) {
  let this_frame = game.this_frame();

  let query = game.world.query_mut::<&Expiry>().with::<IsMob>();
  let events = query
    .into_iter()
    .filter(|(_, expiry)| expiry.0 < this_frame)
    .map(|(mob, _)| MobDespawn {
      mob,
      ty: MobDespawnType::Expired,
    })
    .collect::<SmallVec<[_; 8]>>();

  for event in events {
    game.dispatch(event);
    game.despawn(event.mob);
  }
}
