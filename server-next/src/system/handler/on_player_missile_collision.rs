use airmash_protocol::MobType;
use airmash_protocol::PlaneType;
use smallvec::SmallVec;

use crate::component::*;
use crate::event::PlayerKilled;
use crate::event::PlayerMissileCollision;
use crate::resource::Config;
use crate::AirmashWorld;

#[handler(priority = crate::priority::MEDIUM)]
fn damage_player(event: &PlayerMissileCollision, game: &mut AirmashWorld) {
  let query = game
    .world
    .query_one_mut::<(&MobType, &Owner, &IsMissile)>(event.missile);
  let (&mob, &owner, _) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  let config = game.resources.read::<Config>();
  let minfo = match &config.mobs[mob].missile {
    Some(info) => info,
    None => return,
  };

  let mut events = SmallVec::<[_; 16]>::new();
  let mut query = game
    .world
    .query::<(&mut Health, &PlaneType, &Powerup, &Upgrades)>()
    .with::<IsPlayer>();
  for (player, (health, &plane, powerup, upgrades)) in query.iter() {
    let pinfo = &config.planes[plane];

    // No damage can be done if the player is shielded
    if powerup.shield() {
      continue;
    }

    health.0 -= minfo.damage * pinfo.damage_factor
      / config.upgrades.defense.factor[upgrades.defense as usize];

    if health.0 <= 0.0 {
      events.push(PlayerKilled {
        missile: event.missile,
        player,
        killer: owner.0,
      });
    }
  }
}

#[handler]
fn send_player_hit(event: &PlayerMissileCollision, game: &mut AirmashWorld) {
  use crate::protocol::server::{PlayerHit, PlayerHitPlayer};

  let query = game
    .world
    .query_one_mut::<(&MobType, &Owner, &Position, &IsMissile)>(event.missile);
  let (&mob, &owner, &pos, _) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  let players = event
    .players
    .iter()
    .filter_map(|&player| {
      let query = game.world.query_one::<(&Health, &HealthRegen)>(player);
      let mut query = match query {
        Ok(query) => query.with::<IsPlayer>(),
        Err(_) => return None,
      };

      query.get().map(|(health, regen)| PlayerHitPlayer {
        id: player.id() as _,
        health: health.0,
        health_regen: regen.0,
      })
    })
    .collect();

  let packet = PlayerHit {
    id: event.missile.id() as _,
    owner: owner.0.id() as _,
    pos: pos.0,
    ty: mob,
    players,
  };

  game.send_to_visible(packet.pos, packet);
}
