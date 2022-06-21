use std::borrow::Cow;
use std::time::Duration;

use protocol::{PlaneType, Vector2};
use serde::{Deserialize, Serialize};

use crate::util::duration;
use crate::{MissilePrototype, PrototypeRef, PtrRef, SpecialPrototype, StringRef, ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(
  serialize = "
    Ref::MissileRef: Serialize,
    Ref::SpecialRef: Serialize,
    Ref::PowerupRef: Serialize,
    Ref::PlaneRef: Serialize,
    Ref::MobRef: Serialize,
  ",
  deserialize = "
    Ref::MissileRef: Deserialize<'de>,
    Ref::SpecialRef: Deserialize<'de>,
    Ref::PowerupRef: Deserialize<'de>,
    Ref::PlaneRef: Deserialize<'de>,
    Ref::MobRef: Deserialize<'de>,
  "
))]
pub struct PlanePrototype<'a, Ref: PrototypeRef<'a>> {
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
  pub special: Ref::SpecialRef,

  /// Name of the missile that this plane will fire. This will correspond to a
  /// named MissilePrototype instance which will be used to determine the type
  /// of the fired missile.
  pub missile: Ref::MissileRef,

  /// The offset at which the missile will be fired from the plane. X
  /// corresponds to the distance in front of the plane while Y gives the
  /// distance sideways from the plane and will alternate sides with each shot.
  pub missile_offset: Vector2<f32>,

  /// The energy that it takes the plane to fire a single shot.
  pub fire_energy: f32,

  /// The minimum delay between firing two consecutive shots.
  #[serde(with = "duration")]
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

  /// Displacement of the outside missile when the plane fires with an inferno.
  pub inferno_offset: Vector2<f32>,
  /// Angle of the outside missile when the plane fires with an inferno.
  pub inferno_angle: f32,
}

impl PlanePrototype<'_, StringRef> {
  pub const fn predator() -> Self {
    Self {
      name: Cow::Borrowed("predator"),
      server_type: PlaneType::Predator,
      special: Cow::Borrowed("boost"),
      missile: Cow::Borrowed("predator"),
      missile_offset: Vector2::new(35.0, 0.0),
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
      inferno_offset: Vector2::new(18.0, 1.25),
      inferno_angle: 0.05,
    }
  }

  pub const fn tornado() -> Self {
    Self {
      name: Cow::Borrowed("tornado"),
      server_type: PlaneType::Tornado,
      special: Cow::Borrowed("multishot"),
      missile: Cow::Borrowed("tornado-single"),
      missile_offset: Vector2::new(40.0, 0.0),
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
      inferno_offset: Vector2::new(15.1, 10.0),
      inferno_angle: 0.05,
    }
  }

  pub const fn prowler() -> Self {
    Self {
      name: Cow::Borrowed("prowler"),
      server_type: PlaneType::Prowler,
      special: Cow::Borrowed("stealth"),
      missile: Cow::Borrowed("prowler"),
      missile_offset: Vector2::new(35.0, 0.0),
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
      inferno_offset: Vector2::new(18.0, 2.25),
      inferno_angle: 0.05,
    }
  }

  pub const fn mohawk() -> Self {
    Self {
      name: Cow::Borrowed("mohawk"),
      server_type: PlaneType::Mohawk,
      special: Cow::Borrowed("strafe"),
      missile: Cow::Borrowed("mohawk"),
      missile_offset: Vector2::new(10.0, 15.0),
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
      inferno_offset: Vector2::new(0.0, 0.0),
      inferno_angle: 0.1,
    }
  }

  pub const fn goliath() -> Self {
    Self {
      name: Cow::Borrowed("goliath"),
      server_type: PlaneType::Goliath,
      special: Cow::Borrowed("repel"),
      missile: Cow::Borrowed("goliath"),
      missile_offset: Vector2::new(35.0, 0.0),
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
      inferno_offset: Vector2::new(30.0, 2.1),
      inferno_angle: 0.04,
    }
  }
}

impl PlanePrototype<'_, StringRef> {
  pub(crate) fn resolve<'a>(
    self,
    missiles: &'a [MissilePrototype],
    specials: &'a [SpecialPrototype<'a, PtrRef>],
  ) -> Result<PlanePrototype<'a, PtrRef>, ValidationError> {
    if self.name.is_empty() {
      return Err(ValidationError::custom(
        "name",
        "plane prototype had an empty name",
      ));
    }

    let missile =
      missiles
        .iter()
        .find(|m| m.name == self.missile)
        .ok_or(ValidationError::custom(
          "missile",
          format_args!(
            "plane prototype refers to a nonexistant missile prototype `{}`",
            self.missile
          ),
        ))?;
    let special =
      specials
        .iter()
        .find(|s| s.name == self.special)
        .ok_or(ValidationError::custom(
          "special",
          format_args!(
            "plane prototype refers to nonexistant special prototype `{}`",
            self.special
          ),
        ))?;

    // FIXME: Once <https://github.com/rust-lang/rust/issues/86555> stabilizes we can replace this with
    //        Ok(PlanePrototype { missile, special, ..self })
    Ok(PlanePrototype {
      missile,
      special,
      name: self.name,
      server_type: self.server_type,
      missile_offset: self.missile_offset,
      fire_energy: self.fire_energy,
      fire_delay: self.fire_delay,
      damage_factor: self.damage_factor,
      max_speed: self.max_speed,
      min_speed: self.min_speed,
      flag_speed: self.flag_speed,
      inferno_factor: self.inferno_factor,
      health_regen: self.health_regen,
      energy_regen: self.energy_regen,
      turn_factor: self.turn_factor,
      accel: self.accel,
      brake: self.brake,
      inferno_offset: self.inferno_offset,
      inferno_angle: self.inferno_angle,
    })
  }
}
