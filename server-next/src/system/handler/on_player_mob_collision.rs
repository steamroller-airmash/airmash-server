use crate::component::*;
use crate::event::{MobDespawn, MobDespawnType, PlayerMobCollision};
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
  let (&mob, _) = match game.world.query_one_mut::<(&MobType, &IsMob)>(event.mob) {
    Ok(query) => query,
    Err(_) => return,
  };

  if mob != MobType::Upgrade {
    return;
  }

  let (upgrades, _) = match game
    .world
    .query_one_mut::<(&mut Upgrades, &IsPlayer)>(event.player)
  {
    Ok(query) => query,
    Err(_) => return,
  };

  upgrades.unused += 1;
}

#[handler]
fn send_player_upgrade(event: &PlayerMobCollision, game: &mut AirmashGame) {
  use crate::protocol::server::ScoreUpdate;

  let (&mob, _) = match game.world.query_one_mut::<(&MobType, &IsMob)>(event.mob) {
    Ok(query) => query,
    Err(_) => return,
  };

  if mob != MobType::Upgrade {
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
    total_deaths: deaths.0
  };
  game.send_to(event.player, packet);
}
