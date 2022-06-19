use std::collections::HashSet;

use smallvec::SmallVec;

use crate::component::*;
use crate::config::{MissilePrototypeRef, PlanePrototypeRef};
use crate::event::{PlayerHit, PlayerKilled, PlayerMissileCollision};
use crate::resource::{Config, GameConfig};
use crate::AirmashGame;

#[handler(priority = crate::priority::MEDIUM)]
fn damage_player(event: &PlayerMissileCollision, game: &mut AirmashGame) {
  let query = game
    .world
    .query_one_mut::<(&MissilePrototypeRef, &Owner, &IsMissile)>(event.missile);
  let (&mob, &owner, _) = match query {
    Ok(query) => query,
    Err(_) => return,
  };

  let game_config = game.resources.read::<GameConfig>();
  let config = game.resources.read::<Config>();
  let attacker = game.world.get::<IsPlayer>(owner.0).ok().map(|_| owner.0);

  let mut events = SmallVec::<[_; 16]>::new();
  let mut hits = SmallVec::<[_; 16]>::new();
  let mut killed = HashSet::new();
  for player in event.players.iter().copied() {
    let query = game.world.query_one::<(
      &mut Health,
      &PlanePrototypeRef,
      &Powerup,
      &Upgrades,
      &mut IsAlive,
    )>(player);
    let mut query = match query {
      Ok(query) => query.with::<IsPlayer>(),
      Err(_) => continue,
    };

    if let Some((health, &plane, powerup, upgrades, alive)) = query.get() {
      // No damage can be done if the player is dead
      if !alive.0 {
        continue;
      }

      // No damage can be done if the player is shielded
      if powerup.shield() {
        hits.push(PlayerHit {
          player,
          missile: event.missile,
          damage: 0.0,
          attacker,
        });

        continue;
      }

      let damage = match game_config.allow_damage {
        true => {
          mob.damage * plane.damage_factor
            / config.upgrades.defense.factor[upgrades.defense as usize]
        }
        false => 0.0,
      };
      health.0 -= damage;

      hits.push(PlayerHit {
        player,
        missile: event.missile,
        damage,
        attacker,
      });

      if health.0 <= 0.0 {
        // Avoid double-kills if multiple missiles hit the player in the same frame.
        if !killed.insert(player) {
          continue;
        }

        let owner = game.world.get::<IsPlayer>(owner.0).ok().map(|_| owner.0);

        events.push(PlayerKilled {
          missile: event.missile,
          player,
          killer: owner,
        });
      }
    }
  }

  drop(config);
  drop(game_config);

  game.dispatch_many(hits);
  game.dispatch_many(events);
}

#[handler]
fn send_player_hit(event: &PlayerMissileCollision, game: &mut AirmashGame) {
  use crate::protocol::server::{PlayerHit, PlayerHitPlayer};

  let query = game
    .world
    .query_one_mut::<(&MissilePrototypeRef, &Owner, &Position, &IsMissile)>(event.missile);
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
    ty: mob.server_type,
    players,
  };

  game.send_to_visible(packet.pos, packet);
}
