//! Components used within airmash.

use std::time::{Duration, Instant};

use airmash_protocol::Vector2;
use bstr::BString;
use hecs::Entity;
use uuid::Uuid;

mod keystate;

pub use self::keystate::KeyState;
pub use crate::protocol::{FlagCode, MobType, PlaneType, PowerupType};

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

  /// The total number of times that a player has captured a flag.
  ///
  /// Mainly for use within CTF.
  pub type Captures = u32;

  /// The number of times a player has killed another player.
  pub type KillCount = u32;

  /// The number of times that a player has been killed by another player.
  pub type DeathCount = u32;

  /// The total amount of damage that a player has dealt.
  pub type TotalDamage = f32;

  /// The current ping of a player.
  pub type PlayerPing = Duration;

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

  /// Whether a player is currently alive.
  pub type IsAlive = bool;

  /// Whether a player is currently spectating.
  pub type IsSpectating = bool;

  /// Whether a player's special is currently active.
  ///
  /// Note that this component really only has meaning for prowlers and
  /// mohawks.
  pub type SpecialActive = bool;

  /// Whether a player is currently allowed to respawn.
  pub type RespawnAllowed = bool;

  /// A unique ID corresponding to the current player connection ID.
  pub type Session = Uuid;

  /// The player that currently owns a missile.
  ///
  /// Normally this corresponds to the player that fired the missile but if
  /// the missile is reflected by a goliath then the owner will change to
  /// the player that reflected it.
  pub type Owner = Entity;
}

/// Marker component indicating that an entity is a player.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsPlayer;

/// Marker component indicating that an entity is a missile.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMissile;

/// Marker component indicating that an entity is a mob.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsMob;

/// Marker component indicating that an entity is a zombie.
///
/// There really should never be a reason for this to need to be used as zombie
/// components are just meant to prevent entity IDs from being reused within a
/// short period of time.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IsZombie;

/// Data on the current powerup in use by a player.
///
/// This type is not used as a component, see [`Powerup`] instead.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PowerupData {
  pub ty: PowerupType,
  pub end_time: Instant,
}

/// The current powerup that a player has, or none if the player has no powerup.
///
/// Utility methods are provided to simplify checking what powerups a player
/// has.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Powerup {
  pub data: Option<PowerupData>,
}

impl Powerup {
  pub fn none() -> Self {
    Self::default()
  }

  /// Create a new powerup with the provided type and expiry time.
  pub fn new(ty: PowerupType, end_time: Instant) -> Self {
    Self {
      data: Some(PowerupData { ty, end_time }),
    }
  }

  /// Whether the current powerup is an inferno.
  pub fn inferno(&self) -> bool {
    self
      .data
      .map(|x| x.ty == PowerupType::Inferno)
      .unwrap_or(false)
  }

  /// Whether the current powerup is a shield.
  pub fn shield(&self) -> bool {
    self
      .data
      .map(|x| x.ty == PowerupType::Shield)
      .unwrap_or(false)
  }

  /// The time at which the current powerup expires, should there be one.
  pub fn expires(&self) -> Option<Instant> {
    self.data.map(|x| x.end_time)
  }
}

/// The current state of the upgrades that a player has.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Upgrades {
  pub speed: u8,
  pub defense: u8,
  pub energy: u8,
  pub missile: u8,
  pub unused: u16,
}

/// The previous state of the upgrades that a player has.
///
/// If this differs from the current state then the server will automatically
/// send an update packet to the client.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrevUpgrades(pub Upgrades);

/// Trajectory info for a missile.
///
/// This is used to delete missiles which have reached the end of their
/// lifetime.
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

/// The side from which the next missile from this player will be fired. This
/// alternates every time the player fires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MissileFiringSide {
  Left,
  Right,
}

impl MissileFiringSide {
  pub fn reverse(self) -> Self {
    match self {
      Self::Left => Self::Right,
      Self::Right => Self::Left,
    }
  }

  pub fn multiplier(self) -> f32 {
    match self {
      Self::Left => -1.0,
      Self::Right => 1.0,
    }
  }
}
