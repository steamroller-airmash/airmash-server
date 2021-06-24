//! Components used within airmash

use std::time::Instant;
use crate::protocol::PowerupType;

mod keystate;

pub use self::keystate::KeyState;

def_wrappers!{
  pub type Position = crate::protocol::Position;
  pub type Velocity = crate::protocol::Velocity;
  pub type Rotation = crate::protocol::Rotation;
  pub type Energy = crate::protocol::Energy;
  pub type Health = crate::protocol::Health;
  pub type EnergyRegen = crate::protocol::EnergyRegen;
  pub type HealthRegen = crate::protocol::HealthRegen;
  pub type Team = crate::protocol::Team;

  pub type LastUpdateTime = Instant;
  pub type LastSpecialTime = Instant;
  pub type SpecialActive = bool;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsPlayer;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMissile;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMob;
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsAlive;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Powerup {
  pub ty: PowerupType,
  pub end_time: Instant
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Upgrades {
  pub speed: u8,
  pub defense: u8,
  pub energy: u8,
  pub missile: u8,
  pub unused: u16,
}
