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
fn update_player_upgrades(event: &PlayerMobCollision, game: &mut AirmashGame) {
  let (&mob, _) = match game
    .world
    .query_one_mut::<(&MobPrototypeRef, &IsMob)>(event.mob)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if mob.server_type != MobType::Upgrade {
    return;
  }

  let (upgrades, prev, _) = match game
    .world
    .query_one_mut::<(&mut Upgrades, &mut PrevUpgrades, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  upgrades.unused += 1;
  prev.0 = *upgrades;
}

#[handler]
fn send_player_upgrade(event: &PlayerMobCollision, game: &mut AirmashGame) {
  use crate::protocol::server::ScoreUpdate;

  let (&mob, _) = match game
    .world
    .query_one_mut::<(&MobPrototypeRef, &IsMob)>(event.mob)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if mob.server_type != MobType::Upgrade {
    return;
  }

  let (upgrades, score, earnings, kills, deaths, _) = match game.world.query_one_mut::<(
    &Upgrades,
    &Score,
    &Earnings,
    &KillCount,
    &DeathCount,
    &IsPlayer,
  )>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  let packet = ScoreUpdate {
    id: event.player.id() as _,
    upgrades: upgrades.unused,
    score: score.0,
    earnings: earnings.0,
    total_kills: kills.0,
    total_deaths: deaths.0,
  };
  game.send_to(event.player, packet);
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

  if !mob.server_type.is_powerup() {
    return;
  }

  let (effects, _) = match game
    .world
    .query_one_mut::<(&Effects, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  if effects.expiry().is_some() {
    game.dispatch(PowerupExpire {
      player: event.player,
    });
  }

  game.dispatch(PlayerPowerup {
    player: event.player,
    powerup: mob.powerup,
  });
}
