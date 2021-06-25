//! Components used within airmash

use crate::protocol::PowerupType;
use bstr::BString;
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
  pub type KillCount = u16;
  pub type DeathCount = u16;

  pub type LastUpdateTime = Instant;
  pub type LastSpecialTime = Instant;
  pub type SpecialActive = bool;
  pub type IsAlive = bool;

  pub type Session = Uuid;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsPlayer;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMissile;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMob;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Powerup {
  pub ty: PowerupType,
  pub end_time: Instant,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Upgrades {
  pub speed: u8,
  pub defense: u8,
  pub energy: u8,
  pub missile: u8,
  pub unused: u16,
}

impl From<IsAlive> for crate::protocol::PlayerStatus {
  fn from(x: IsAlive) -> Self {
    match x.0 {
      true => crate::protocol::PlayerStatus::Alive,
      false => crate::protocol::PlayerStatus::Dead,
    }
  }
}
