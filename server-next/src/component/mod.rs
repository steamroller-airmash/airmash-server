//! Components used within airmash

use crate::protocol::PowerupType;
use airmash_protocol::Vector2;
use bstr::BString;
use hecs::Entity;
use std::time::Instant;
use uuid::Uuid;

mod keystate;

pub use self::keystate::KeyState;

def_wrappers! {
  pub type Position = crate::protocol::Position;
  pub type Velocity = crate::protocol::Velocity;
  pub type Rotation = crate::protocol::Rotation;
  pub type Energy = crate::protocol::Energy;
  pub type Health = crate::protocol::Health;
  pub type EnergyRegen = crate::protocol::EnergyRegen;
  pub type HealthRegen = crate::protocol::HealthRegen;
  pub type Team = crate::protocol::Team;
  pub type Level = crate::protocol::Level;
  ##[nocopy]
  pub type Name = BString;
  pub type Score = u32;
  pub type Earnings = u32;
  pub type KillCount = u32;
  pub type DeathCount = u32;

  pub type LastUpdateTime = Instant;
  pub type LastSpecialTime = Instant;
  pub type LastFireTime = Instant;
  pub type LastActionTime = Instant;
  pub type Expiry = Instant;

  /// The time at which a missile spawned.
  pub type SpawnTime = Instant;

  pub type SpecialActive = bool;
  pub type IsAlive = bool;
  pub type RespawnAllowed = bool;

  pub type Session = Uuid;
  pub type Owner = Entity;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsPlayer;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMissile;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMob;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsZombie;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PowerupData {
  pub ty: PowerupType,
  pub end_time: Instant,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Powerup {
  pub data: Option<PowerupData>,
}

impl Powerup {
  pub fn inferno(&self) -> bool {
    self
      .data
      .map(|x| x.ty == PowerupType::Inferno)
      .unwrap_or(false)
  }

  pub fn shield(&self) -> bool {
    self
      .data
      .map(|x| x.ty == PowerupType::Shield)
      .unwrap_or(false)
  }

  pub fn expires(&self) -> Option<Instant> {
    self.data.map(|x| x.end_time)
  }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Upgrades {
  pub speed: u8,
  pub defense: u8,
  pub energy: u8,
  pub missile: u8,
  pub unused: u16,
}

#[derive(Copy, Clone, Debug)]
pub struct MissileTrajectory {
  pub start: Vector2<f32>,
  pub maxdist: f32,
}

impl From<IsAlive> for crate::protocol::PlayerStatus {
  fn from(x: IsAlive) -> Self {
    match x.0 {
      true => crate::protocol::PlayerStatus::Alive,
      false => crate::protocol::PlayerStatus::Dead,
    }
  }
}
