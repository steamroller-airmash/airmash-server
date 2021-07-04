//! Components used within airmash.

use crate::protocol::PowerupType;
use airmash_protocol::Vector2;
use bstr::BString;
use hecs::Entity;
use std::time::Instant;
use uuid::Uuid;

mod keystate;

pub use self::keystate::KeyState;

pub use crate::protocol::{FlagCode, MobType, PlaneType};

def_wrappers! {
  /// The position of an entity.
  pub type Position = crate::protocol::Position;

  /// The velocity of an entity.
  pub type Velocity = crate::protocol::Velocity;

  /// The acceleration of an entity.
  pub type Accel = crate::protocol::Accel;

  /// The rotation of an entity.
  pub type Rotation = crate::protocol::Rotation;

  /// The amount of energy that a player has.
  ///
  /// Ranges from 0 to 1.
  pub type Energy = crate::protocol::Energy;

  /// The amount of health that a player has.
  ///
  /// Ranges from 0 to 1.
  pub type Health = crate::protocol::Health;

  /// The rate at which a player's energy regenerates.
  pub type EnergyRegen = crate::protocol::EnergyRegen;

  /// The rate at which a player's health regenerates.
  pub type HealthRegen = crate::protocol::HealthRegen;

  /// The team that a player and/or missile belongs to.
  pub type Team = crate::protocol::Team;

  /// The level of a player.
  pub type Level = crate::protocol::Level;

  /// The name of a player.
  ##[nocopy]
  pub type Name = BString;

  /// The current score of a player.
  pub type Score = u32;

  /// The sum of all the score that the player has ever earned.
  pub type Earnings = u32;

  /// The number of times a player has killed another player.
  pub type KillCount = u32;

  /// The number of times that a player has been killed by another player.
  pub type DeathCount = u32;

  /// The current ping of a player.
  pub type PlayerPing = u16;

  /// The player that another spectating player is watching.
  #[derive(Default)]
  pub type Spectating = Option<Entity>;

  /// The time at which the last [`PlayerUpdate`] packet was sent.
  ///
  /// This can also be used to force a [`PlayerUpdate`] packet to be sent when
  /// that is needed.
  ///
  /// [`PlayerUpdate`]: crate::protocol::server::PlayerUpdate
  pub type LastUpdateTime = Instant;

  /// The time at which the player last used their special.
  ///
  /// Note that this only really applies for specials that have a time
  /// restriction such as the goliath repel.
  pub type LastSpecialTime = Instant;

  /// The time at which a player last fired.
  pub type LastFireTime = Instant;

  /// The time at which a player last performed any action.
  pub type LastActionTime = Instant;

  /// The time at which a zombie entity will be deleted.
  pub type Expiry = Instant;

  /// The time at which a missile spawned.
  pub type SpawnTime = Instant;
  /// The time at which a player joined.
  pub type JoinTime = Instant;

  pub type IsAlive = bool;
  pub type IsSpectating = bool;
  pub type SpecialActive = bool;
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
  pub fn none() -> Self {
    Self::default()
  }

  pub fn new(ty: PowerupType, end_time: Instant) -> Self {
    Self {
      data: Some(PowerupData { ty, end_time }),
    }
  }

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
