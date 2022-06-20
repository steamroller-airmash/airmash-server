use std::borrow::Cow;

use protocol::MobType;
use serde::{Deserialize, Serialize};

use crate::ValidationError;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissilePrototype {
  /// The name with which to refer to this missile prototype. It must be unique
  /// among all missile prototypes.
  pub name: Cow<'static, str>,

  /// The mob type that will be communicated to the client.
  ///
  /// This will determine the entity type that the client will show for the mob.
  /// Setting this to a non-missile type will not result in the client showing
  /// those mob types as moving mobs.
  pub server_type: MobType,

  /// The maximum speed at which this missile can travel.
  pub max_speed: f32,
  /// The base speed that this missile would be fired at if the plane firing it
  /// was not moving at all.
  pub base_speed: f32,
  /// The fraction of the speed of the parent that the missile inherits when it
  /// is fired.
  pub inherit_factor: f32,
  /// The rate of acceleration of the missile.
  pub accel: f32,
  /// The amount of damage that this missile would do to a goliath.
  pub damage: f32,
  /// The maximum distance that this missile will travel before it despawns.
  pub distance: f32,
}

impl MissilePrototype {
  pub const fn predator() -> Self {
    Self {
      name: Cow::Borrowed("predator"),
      server_type: MobType::PredatorMissile,
      max_speed: 9.0,
      base_speed: 4.05,
      inherit_factor: 0.3,
      accel: 0.105,
      damage: 0.4,
      distance: 1104.0,
    }
  }

  pub const fn tornado() -> Self {
    Self {
      name: Cow::Borrowed("tornado-single"),
      server_type: MobType::TornadoSingleMissile,
      max_speed: 7.0,
      base_speed: 3.5,
      inherit_factor: 0.3,
      accel: 0.0875,
      damage: 0.4,
      distance: 997.0,
    }
  }

  pub const fn prowler() -> Self {
    Self {
      name: Cow::Borrowed("prowler"),
      server_type: MobType::ProwlerMissile,
      max_speed: 7.0,
      base_speed: 2.8,
      inherit_factor: 0.3,
      accel: 0.07,
      damage: 0.45,
      distance: 819.0,
    }
  }

  pub const fn mohawk() -> Self {
    Self {
      name: Cow::Borrowed("mohawk"),
      server_type: MobType::MohawkMissile,
      max_speed: 9.0,
      base_speed: 5.7,
      inherit_factor: 0.3,
      accel: 0.14,
      damage: 0.2,
      distance: 1161.0,
    }
  }

  pub const fn goliath() -> Self {
    Self {
      name: Cow::Borrowed("goliath"),
      server_type: MobType::GoliathMissile,
      max_speed: 6.0,
      base_speed: 2.1,
      inherit_factor: 0.3,
      accel: 0.0375,
      damage: 1.2,
      distance: 1076.0,
    }
  }

  pub const fn tornado_triple() -> Self {
    Self {
      name: Cow::Borrowed("tornado-triple"),
      server_type: MobType::TornadoTripleMissile,
      max_speed: 7.0,
      base_speed: 3.5,
      inherit_factor: 0.3,
      accel: 0.0875,
      damage: 0.3,
      distance: 581.0,
    }
  }
}

impl MissilePrototype {
  pub(crate) fn resolve(self) -> Result<Self, ValidationError> {
    if self.name.is_empty() {
      return Err(ValidationError::custom("name", "prototype had empty name"));
    }

    Ok(self)
  }
}
