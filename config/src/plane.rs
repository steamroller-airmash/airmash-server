use crate::CowString;
use protocol::{PlaneType, Vector2};
use std::{borrow::Cow, time::Duration};

#[derive(Clone, Debug)]
pub struct PlaneProto {
  pub name: CowString,
  /// The type of this plane as communicated to clients.
  pub server_type: PlaneType,

  // Rotation
  /// The rate at which a plane is able to turn in radians/frame.
  pub turn_rate: f32,

  // Acceleration
  /// The rate at which a player is able to accelerate (in units/frame).
  pub accel: f32,
  /// The rate at which a player brakes when they are not pressing any keys (in
  /// units/frame).
  pub brake: f32,

  // Speeds
  /// The maximum speed at which a player can travel (in units/frame).
  pub max_speed: f32,
  /// The minimum speed at which a player can travel before their speed gets
  /// truncated to 0.
  pub min_speed: f32,

  // Regen
  /// The rate at which a player's health regenerates (in units/frame).
  pub health_regen: f32,
  /// The rate at which a player's energy regenerates (in units/frame).
  pub energy_regen: f32,

  // Health
  /// A scaling factor that indicates the amount of damage that missiles do to
  /// this plane type. This is normalized so that a goliath has a damage factor
  /// of 1. Larger values means missiles do more damage so, e.g., for predators,
  /// which have a damage factor of 2, missiles will do twice the damage that
  /// they would have done to a goliath.
  pub damage_factor: f32,

  // Energy Requirement
  /// The energy requirement for this plane type to fire a missile.
  pub fire_energy: f32,
  /// The minimum delay between firing missiles. Even if the plane has enough
  /// energy to fire a missile it must wait at least this amount of time.
  pub fire_delay: Duration,
  /// The offset at which a missile is fired by the plane. If an x offset is
  /// provided then the side from which the missile is shot will alternate with
  /// each missile fired.
  ///
  /// Multishots will be rotate with distance equal to the y distance provided
  /// and will use the position (x, 0.0) in the coordinate frame of the plane as
  /// their base of rotation.
  pub fire_offset: Vector2<f32>,

  // Referenced Prototypes
  /// The name of the prototype of the special action that this plane can take.
  pub special: CowString,
  /// The name of the prototype of the missile that this plane can shoot.
  pub missile: CowString,
}

impl PlaneProto {
  /// Create the default configuration for a predator.
  pub const fn predator() -> Self {
    Self {
      name: Cow::Borrowed("predator"),
      server_type: PlaneType::Predator,

      turn_rate: 0.065,

      accel: 0.225,
      brake: 0.025,

      max_speed: 5.5,
      min_speed: 0.001,

      health_regen: 0.001,
      energy_regen: 0.008,

      damage_factor: 2.0,

      fire_energy: 0.6,
      fire_delay: Duration::from_millis(550),
      fire_offset: Vector2::new(0.0, 25.0),

      special: Cow::Borrowed("boost"),
      missile: Cow::Borrowed("predator"),
    }
  }

  /// Create the default configuration for a goliath.
  pub const fn goliath() -> Self {
    Self {
      name: Cow::Borrowed("goliath"),
      server_type: PlaneType::Goliath,

      turn_rate: 0.04,

      accel: 0.15,
      brake: 0.015,

      max_speed: 3.5,
      min_speed: 0.001,

      health_regen: 0.0005,
      energy_regen: 0.005,

      damage_factor: 1.0,

      fire_energy: 0.9,
      fire_delay: Duration::from_millis(300),
      fire_offset: Vector2::new(0.0, 35.0),

      special: Cow::Borrowed("repel"),
      missile: Cow::Borrowed("goliath"),
    }
  }

  /// Create the default configuration for a mohawk.
  pub const fn mohawk() -> Self {
    Self {
      name: Cow::Borrowed("mohawk"),
      server_type: PlaneType::Mohawk,

      turn_rate: 0.07,

      accel: 0.275,
      brake: 0.025,

      max_speed: 6.0,
      min_speed: 0.001,

      health_regen: 0.001,
      energy_regen: 0.01,

      damage_factor: 2.6375,

      fire_energy: 0.3,
      fire_delay: Duration::from_millis(300),
      // TODO: Determine mohawk horizontal offset
      fire_offset: Vector2::new(0.0, 10.0),

      special: Cow::Borrowed("strafe"),
      missile: Cow::Borrowed("mohawk"),
    }
  }

  /// Create the default configuration for a tornado.
  pub const fn tornado() -> Self {
    Self {
      name: Cow::Borrowed("tornado"),
      server_type: PlaneType::Tornado,

      turn_rate: 0.055,
      accel: 0.2,
      brake: 0.025,

      max_speed: 4.5,
      min_speed: 0.001,

      health_regen: 0.001,
      energy_regen: 0.006,

      // 5.0 / 3.0
      damage_factor: 1.66666666667,

      fire_energy: 0.5,
      fire_delay: Duration::from_millis(500),
      fire_offset: Vector2::new(0.0, 40.0),

      special: Cow::Borrowed("triple-shot"),
      missile: Cow::Borrowed("tornado-single"),
    }
  }

  /// Create the default configuration for a prowler.
  pub const fn prowler() -> Self {
    Self {
      name: Cow::Borrowed("prowler"),
      server_type: PlaneType::Prowler,

      turn_rate: 0.055,
      accel: 0.2,
      brake: 0.025,

      max_speed: 4.5,
      min_speed: 0.001,

      health_regen: 0.001,
      energy_regen: 0.006,

      // 5.0 / 3.0
      damage_factor: 1.66666666667,

      fire_energy: 0.75,
      fire_delay: Duration::from_millis(300),
      fire_offset: Vector2::new(0.0, 35.0),

      special: Cow::Borrowed("stealth"),
      missile: Cow::Borrowed("prowler"),
    }
  }
}
