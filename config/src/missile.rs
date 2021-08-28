use std::borrow::Cow;

use crate::CowString;
use protocol::MobType;

#[derive(Clone, Debug)]
pub struct MissileProto {
  pub name: CowString,
  /// The type of this missile as communicated to clients.
  pub server_type: MobType,

  /// The rate at which the missile accelerates (in units/frame^2).
  pub accel: f32,

  /// The maximum speed at which the missile can travel (in units/frame).
  ///
  /// Note that can be modified by various effects (e.g. if the player has
  /// upgraded missile speed) so it is not the _true_ maximum speed of the
  /// missile under all conditions.
  pub max_speed: f32,

  /// The minimum at which the missile will travel when fired from a plane.
  /// Since the missile gains some speed from the direction the plane is moving
  /// it could be launched at a greater speed but if it would be launched at a
  /// lower speed then this one will be used instead.
  pub base_speed: f32,

  /// The factor which the player's speed adds on to this missile's speed when
  /// fired. 0 means that it has no effect, 1 means that the missile will be
  /// launched at the same velocity as the player (with the exception of
  /// base_speed limits).
  pub launch_factor: f32,

  /// The damage this missile would do to a goliath.
  pub damage: f32,

  /// The maximum distance that this missile can travel before it is despawned.
  pub max_distance: f32,
}

impl MissileProto {
  pub const fn predator() -> Self {
    Self {
      name: Cow::Borrowed("predator"),
      server_type: MobType::PredatorMissile,

      accel: 0.105,
      max_speed: 9.0,
      base_speed: 4.05,
      launch_factor: 0.3,
      damage: 0.4,
      max_distance: 1104.0,
    }
  }

  pub const fn goliath() -> Self {
    Self {
      name: Cow::Borrowed("goliath"),
      server_type: MobType::GoliathMissile,

      accel: 0.0375,
      max_speed: 6.0,
      base_speed: 2.1,
      launch_factor: 0.3,
      damage: 1.2,
      max_distance: 1076.0,
    }
  }

  pub const fn mohawk() -> Self {
    Self {
      name: Cow::Borrowed("mohawk"),
      server_type: MobType::MohawkMissile,

      accel: 0.14,
      max_speed: 9.0,
      base_speed: 5.7,
      launch_factor: 0.3,
      damage: 0.2,
      max_distance: 1161.0,
    }
  }

  pub const fn tornado() -> Self {
    Self {
      name: Cow::Borrowed("tornado"),
      server_type: MobType::TornadoSingleMissile,

      accel: 0.0875,
      max_speed: 7.0,
      base_speed: 3.5,
      launch_factor: 0.3,
      damage: 0.4,
      max_distance: 997.0,
    }
  }

  pub const fn tornado_triple() -> Self {
    Self {
      name: Cow::Borrowed("tornado-triple"),
      server_type: MobType::TornadoTripleMissile,

      accel: 0.0875,
      max_speed: 7.0,
      base_speed: 3.5,
      launch_factor: 0.3,
      damage: 0.3,
      max_distance: 581.0,
    }
  }
}
