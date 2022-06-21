//! This module has the default component sets for all entity types. This is
//! meant to make it easier add new ones for use within the main server.
//! (External code can add them in EntitySpawn events if it needs to.)

use std::str::FromStr;
use std::time::{Duration, Instant};

use hecs::EntityBuilder;
use uuid::Uuid;

use crate::component::*;
use crate::config::PlanePrototypeRef;
use crate::protocol::client::Login;
use crate::protocol::{FlagCode, Vector2};

/// Build a player
pub(crate) fn build_default_player(
  login: &Login,
  proto: PlanePrototypeRef,
  start_time: Instant,
  this_frame: Instant,
) -> EntityBuilder {
  let mut builder = EntityBuilder::new();
  builder
    .add(IsPlayer)
    .add(Position(Vector2::zeros()))
    .add(Velocity(Vector2::zeros()))
    .add(Rotation(0.0))
    .add(Energy(1.0))
    .add(Health(1.0))
    .add(EnergyRegen(proto.energy_regen))
    .add(HealthRegen(proto.health_regen))
    .add(proto)
    .add(FlagCode::from_str(&login.flag.to_string()).unwrap_or(FlagCode::UnitedNations))
    .add(Level(0))
    .add(Score(0))
    .add(Earnings(0))
    .add(KillCount(0))
    .add(DeathCount(0))
    .add(Upgrades::default())
    .add(PrevUpgrades::default())
    .add(Name(login.name.clone()))
    .add(Team(0))
    .add(IsAlive(true))
    .add(IsSpectating(false))
    .add(Session(Uuid::new_v4()))
    .add(KeyState::default())
    .add(LastFireTime(start_time))
    .add(LastSpecialTime(start_time))
    .add(LastActionTime(start_time))
    .add(SpecialActive(false))
    .add(RespawnAllowed(true))
    .add(JoinTime(this_frame))
    .add(Spectating::default())
    .add(PlayerPing(Duration::ZERO))
    .add(TotalDamage(0.0))
    .add(Captures(0))
    .add(MissileFiringSide::Left)
    .add(Effects::default());

  builder
}

/// Build a missile.
///
/// This one is smaller since most missile components end up needing the info
/// from the player.
pub(crate) fn build_default_missile() -> EntityBuilder {
  let mut builder = EntityBuilder::new();
  builder.add(IsMissile);

  builder
}
