use std::borrow::Cow;
use std::time::Duration;

use protocol::PowerupType;
use serde::{Deserialize, Serialize};

use crate::{EffectPrototype, ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerupPrototype {
  pub name: Cow<'static, str>,
  pub server_type: Option<PowerupType>,
  #[serde(with = "crate::util::option_duration")]
  pub duration: Option<Duration>,
  pub effects: Vec<EffectPrototype>,
}

impl PowerupPrototype {
  pub fn shield() -> Self {
    Self {
      name: Cow::Borrowed("shield"),
      server_type: Some(PowerupType::Shield),
      duration: Some(Duration::from_secs(10)),
      effects: vec![EffectPrototype::shield(), EffectPrototype::despawn()],
    }
  }

  pub fn spawn_shield() -> Self {
    Self {
      name: Cow::Borrowed("spawn-shield"),
      duration: Some(Duration::from_secs(2)),
      ..Self::shield()
    }
  }

  pub fn inferno() -> Self {
    Self {
      name: Cow::Borrowed("inferno"),
      server_type: Some(PowerupType::Inferno),
      duration: Some(Duration::from_secs(10)),
      effects: vec![
        EffectPrototype::inferno(),
        EffectPrototype::despawn(),
        EffectPrototype::Speed { speed_mult: 0.75 },
      ],
    }
  }

  pub fn upgrade() -> Self {
    Self {
      name: Cow::Borrowed("upgrade"),
      server_type: None,
      duration: None,
      effects: vec![EffectPrototype::upgrade(), EffectPrototype::despawn()],
    }
  }
}

impl PowerupPrototype {
  pub(crate) fn resolve(self) -> Result<Self, ValidationError> {
    if self.name.is_empty() {
      return Err(ValidationError::custom(
        "name",
        "powerup protoype had an empty name",
      ));
    }

    Ok(self)
  }
}
