use std::borrow::Cow;
use std::time::Duration;

use protocol::MobType;
use serde::{Deserialize, Serialize};

use crate::util::duration;
use crate::ValidationError;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MobPrototype {
  /// The name that will be used to this mob.
  pub name: Cow<'static, str>,

  /// The mob type that will be communicated to the client.
  ///
  /// This will determine the entity type that the client will show for the mob.
  /// Setting it to the type of a missile is likely to break things.
  pub server_type: MobType,

  /// How long this mob will stick around before despawning.
  #[serde(with = "duration")]
  pub lifetime: Duration,
}

impl MobPrototype {
  pub const fn inferno() -> Self {
    Self {
      name: Cow::Borrowed("inferno"),
      server_type: MobType::Inferno,
      lifetime: Duration::from_secs(60),
    }
  }

  pub const fn shield() -> Self {
    Self {
      name: Cow::Borrowed("shield"),
      server_type: MobType::Shield,
      lifetime: Duration::from_secs(60),
    }
  }

  pub const fn upgrade() -> Self {
    Self {
      name: Cow::Borrowed("upgrade"),
      server_type: MobType::Upgrade,
      lifetime: Duration::from_secs(60),
    }
  }
}

impl MobPrototype {
  pub fn resolve(self) -> Result<Self, ValidationError> {
    if self.name.is_empty() {
      return Err(ValidationError::custom("name", "prototype had empty name"));
    }

    Ok(self)
  }
}
