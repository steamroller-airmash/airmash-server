use std::borrow::Cow;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::util::option_duration;
use crate::ValidationError;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EffectPrototype {
  pub name: Cow<'static, str>,

  #[serde(with = "option_duration")]
  pub duration: Option<Duration>,

  #[serde(flatten)]
  pub data: EffectPrototypeData,
}

impl EffectPrototype {
  pub const fn shield() -> Self {
    Self {
      name: Cow::Borrowed("shield"),
      duration: Some(Duration::from_secs(10)),
      data: EffectPrototypeData::Shield(ShieldEffectPrototype { damage_mult: 0.0 }),
    }
  }

  pub const fn spawn_shield() -> Self {
    Self {
      name: Cow::Borrowed("spawn-shield"),
      duration: Some(Duration::from_secs(2)),
      data: EffectPrototypeData::Shield(ShieldEffectPrototype { damage_mult: 0.0 }),
    }
  }

  pub const fn invulnerable() -> Self {
    Self {
      name: Cow::Borrowed("invulnerable"),
      duration: None,
      data: EffectPrototypeData::Shield(ShieldEffectPrototype { damage_mult: 0.0 }),
    }
  }

  pub const fn inferno() -> Self {
    Self {
      name: Cow::Borrowed("inferno"),
      duration: Some(Duration::from_secs(10)),
      data: EffectPrototypeData::Inferno,
    }
  }

  pub const fn flag_speed() -> Self {
    Self {
      name: Cow::Borrowed("flag-speed"),
      duration: None,
      data: EffectPrototypeData::FixedSpeed(FixedSpeedEffectPrototype { speed: 5.0 }),
    }
  }

  pub const fn upgrade() -> Self {
    Self {
      name: Cow::Borrowed("upgrade"),
      duration: None,
      data: EffectPrototypeData::Upgrade,
    }
  }
}

impl EffectPrototype {
  pub const fn is_shield(&self) -> bool {
    matches!(self.data, EffectPrototypeData::Shield(_))
  }

  pub const fn as_shield(&self) -> Option<&ShieldEffectPrototype> {
    match &self.data {
      EffectPrototypeData::Shield(shield) => Some(shield),
      _ => None,
    }
  }

  pub const fn is_inferno(&self) -> bool {
    matches!(self.data, EffectPrototypeData::Inferno)
  }

  pub const fn is_fixed_speed(&self) -> bool {
    matches!(self.data, EffectPrototypeData::FixedSpeed(_))
  }

  pub const fn as_fixed_speed(&self) -> Option<&FixedSpeedEffectPrototype> {
    match &self.data {
      EffectPrototypeData::FixedSpeed(speed) => Some(speed),
      _ => None,
    }
  }

  pub const fn is_upgrade(&self) -> bool {
    matches!(self.data, EffectPrototypeData::Upgrade)
  }
}

impl EffectPrototype {
  pub(crate) fn resolve(self) -> Result<Self, ValidationError> {
    if self.name.is_empty() {
      return Err(ValidationError::custom("name", "prototype had empty name"));
    }

    Ok(self)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename = "kebab-case")]
pub enum EffectPrototypeData {
  Shield(ShieldEffectPrototype),
  Inferno,
  FixedSpeed(FixedSpeedEffectPrototype),
  Upgrade,
}

/// Effect that modifies the amount of damage done to a plane when it is hit by
/// a missile.
///
/// This is named after the shield (which sets all damage to 0) but can be used
/// for any effect that needs to modify the damage taken by a plane. If multiple
/// instances of this effect are present the all the damage multipliers will be
/// applied.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShieldEffectPrototype {
  pub damage_mult: f32,
}

/// Effect that sets a plane's speed to a fixed value.
///
/// This is used for the flag speed effect in CTF and will set the corresponding
/// bit in the keystate. Note that setting the speed to anything other than 5
/// will cause client desyncs unless the client has been modified as well.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FixedSpeedEffectPrototype {
  pub speed: f32,
}
