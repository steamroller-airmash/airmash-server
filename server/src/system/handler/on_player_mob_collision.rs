use crate::component::*;
use crate::config::MobPrototypeRef;
use crate::event::{MobDespawn, MobDespawnType, PlayerMobCollision, PlayerPowerup, PowerupExpire};
use crate::AirmashGame;

#[handler]
fn dispatch_despawn_event(event: &PlayerMobCollision, game: &mut AirmashGame) {
  if !game.world.contains(event.mob) {
    return;
  }

  game.dispatch(MobDespawn {
    ty: MobDespawnType::PickUp,
    mob: event.mob,
  });
}

#[handler(priority = crate::priority::HIGH)]
fn update_player_powerup(event: &PlayerMobCollision, game: &mut AirmashGame) {
  let (&mob, _) = match game
    .world
    .query_one_mut::<(&MobPrototypeRef, &IsMob)>(event.mob)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let (effects, _) = match game
    .world
    .query_one_mut::<(&Effects, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if effects.powerup().is_some() {
    game.dispatch(PowerupExpire {
      player: event.player,
    });
  }

  game.dispatch(PlayerPowerup {
    player: event.player,
    powerup: mob.powerup,
  });
}
