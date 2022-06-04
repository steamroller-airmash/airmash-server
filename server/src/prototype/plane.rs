use crate::protocol::PlaneType;
use crate::Vector2;
use nalgebra::vector;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanePrototype {
  /// The name with which to refer to this plane prototype. It must be unique
  /// among all plane prototypes.
  pub name: Cow<'static, str>,

  /// This is the plane type that will be communicated to the client.
  ///
  /// This will determine what the client expects for each of the following:
  ///  - max_speed
  ///  - turn_factor
  ///  - accel
  ///  - brake
  ///
  /// Changing these away from the default expected for the plane type will
  /// result in the plane appearing to jump on the client whenever it receives
  /// an update and the plane's shown position being different than what is on
  /// the server.
  pub server_type: PlaneType,

  /// Name of the special effect that this plane has. This will correspond to a
  /// named SpecialPrototype instance.
  pub special: Cow<'static, str>,

  /// Name of the missile that this plane will fire. This will correspond to a
  /// named MissilePrototype instance which will be used to determine the type
  /// of the fired missile.
  pub missile: Cow<'static, str>,

  /// The offset at which the missile will be fired from the plane. X
  /// corresponds to the distance in front of the plane while Y gives the
  /// distance sideways from the plane and will alternate sides with each shot.
  pub missile_offset: Vector2<f32>,

  /// The energy that it takes the plane to fire a single shot.
  pub fire_energy: f32,

  /// The minimum delay between firing two consecutive shots.
  pub fire_delay: Duration,

  /// Multiplier for missile damage. This is somewhat like the inverse of player
  /// health.
  pub damage_factor: f32,

  /// The maximum speed at which the plane can travel.
  pub max_speed: f32,
  /// The minimum speed at which the plane can travel before its speed is
  /// truncated to 0.
  pub min_speed: f32,
  /// The speed at which the plane travels when it is carrying the flag in
  /// CTF-based game modes.
  pub flag_speed: f32,
  /// Multiplier for speed when a player is carrying an inferno.
  pub inferno_factor: f32,

  /// The amount of health that this plane will regenerate with each frame.
  ///
  /// Note that there are 60 frames per second.
  pub health_regen: f32,
  /// The amount of energy that this plane will regenerate with each frame.
  ///
  /// Note that there are 60 frames per second.
  pub energy_regen: f32,

  /// The rate at which this plane turns.
  pub turn_factor: f32,
  /// The rate at which this plane can accelerate.
  pub accel: f32,
  /// The rate at which this plane slows down when no thrust is being applied.
  pub brake: f32,
}

impl PlanePrototype {
  pub const fn predator() -> Self {
    Self {
      name: Cow::Borrowed("predator"),
      server_type: PlaneType::Predator,
      special: Cow::Borrowed("boost"),
      missile: Cow::Borrowed("predator"),
      missile_offset: vector![35.0, 0.0],
      fire_energy: 0.6,
      fire_delay: Duration::from_millis(550),
      damage_factor: 2.0,
      max_speed: 5.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,
      health_regen: 0.001,
      energy_regen: 0.008,
      turn_factor: 0.065,
      accel: 0.225,
      brake: 0.025,
    }
  }

  pub const fn tornado() -> Self {
    Self {
      name: Cow::Borrowed("tornado"),
      server_type: PlaneType::Tornado,
      special: Cow::Borrowed("multishot"),
      missile: Cow::Borrowed("tornado-single"),
      missile_offset: vector![40.0, 0.0],
      fire_energy: 0.5,
      fire_delay: Duration::from_millis(500),
      damage_factor: 1.6666666,
      max_speed: 4.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,
      health_regen: 0.001,
      energy_regen: 0.006,
      turn_factor: 0.055,
      accel: 0.2,
      brake: 0.025,
    }
  }

  pub const fn prowler() -> Self {
    Self {
      name: Cow::Borrowed("prowler"),
      server_type: PlaneType::Prowler,
      special: Cow::Borrowed("cloak"),
      missile: Cow::Borrowed("prowler"),
      missile_offset: vector![35.0, 0.0],
      fire_energy: 0.75,
      fire_delay: Duration::from_millis(300),
      damage_factor: 1.6666666,
      max_speed: 4.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,
      health_regen: 0.001,
      energy_regen: 0.006,
      turn_factor: 0.055,
      accel: 0.2,
      brake: 0.025,
    }
  }

  pub const fn mohawk() -> Self {
    Self {
      name: Cow::Borrowed("mohawk"),
      server_type: PlaneType::Mohawk,
      special: Cow::Borrowed("strafe"),
      missile: Cow::Borrowed("mohawk"),
      missile_offset: vector![10.0, 15.0],
      fire_energy: 0.3,
      fire_delay: Duration::from_millis(300),
      damage_factor: 2.6375,
      max_speed: 6.0,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,
      health_regen: 0.001,
      energy_regen: 0.01,
      turn_factor: 0.07,
      accel: 0.275,
      brake: 0.025,
    }
  }

  pub const fn goliath() -> Self {
    Self {
      name: Cow::Borrowed("goliath"),
      server_type: PlaneType::Goliath,
      special: Cow::Borrowed("reflect"),
      missile: Cow::Borrowed("goliath"),
      missile_offset: vector![35.0, 0.0],
      fire_energy: 0.9,
      fire_delay: Duration::from_millis(300),
      damage_factor: 1.0,
      max_speed: 3.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,
      health_regen: 0.0005,
      energy_regen: 0.005,
      turn_factor: 0.04,
      accel: 0.15,
      brake: 0.015,
    }
  }
}
