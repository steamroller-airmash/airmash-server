use std::borrow::Cow;
use std::time::Duration;

use protocol::MobType;
use serde::{Deserialize, Serialize};

use crate::util::duration;
use crate::{EffectPrototype, PrototypeRef, PtrRef, StringRef, ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(
  serialize = "
    Ref::MissileRef: Serialize,
    Ref::SpecialRef: Serialize,
    Ref::EffectRef: Serialize,
    Ref::PlaneRef: Serialize,
    Ref::MobRef: Serialize,
  ",
  deserialize = "
    Ref::MissileRef: Deserialize<'de>,
    Ref::SpecialRef: Deserialize<'de>,
    Ref::EffectRef: Deserialize<'de>,
    Ref::PlaneRef: Deserialize<'de>,
    Ref::MobRef: Deserialize<'de>,
  "
))]
pub struct MobPrototype<'a, Ref: PrototypeRef<'a>> {
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

  /// The effects of colliding with this mob.
  pub effects: Vec<Ref::EffectRef>,
}

impl MobPrototype<'_, StringRef> {
  pub fn inferno() -> Self {
    Self {
      name: Cow::Borrowed("inferno"),
      server_type: MobType::Inferno,
      lifetime: Duration::from_secs(60),
      effects: vec![Cow::Borrowed("inferno")],
    }
  }

  pub fn shield() -> Self {
    Self {
      name: Cow::Borrowed("shield"),
      server_type: MobType::Shield,
      lifetime: Duration::from_secs(60),
      effects: vec![Cow::Borrowed("shield")],
    }
  }

  pub fn upgrade() -> Self {
    Self {
      name: Cow::Borrowed("upgrade"),
      server_type: MobType::Upgrade,
      lifetime: Duration::from_secs(60),
      effects: vec![Cow::Borrowed("upgrade")],
    }
  }
}

impl MobPrototype<'_, StringRef> {
  pub fn resolve(
    self,
    effects: &[EffectPrototype],
  ) -> Result<MobPrototype<PtrRef>, ValidationError> {
    if self.name.is_empty() {
      return Err(ValidationError::custom("name", "prototype had empty name"));
    }

    let effects = self
      .effects
      .into_iter()
      .enumerate()
      .map(|(idx, effect)| {
        effects
          .iter()
          .find(|&proto| proto.name == effect)
          .ok_or(ValidationError::custom(
            idx,
            format_args!(
              "mob prototype refers to a nonexistant effect prototype `{}`",
              effect
            ),
          ))
      })
      .collect::<Result<_, ValidationError>>()
      .map_err(|e| e.with("effects"))?;

    Ok(MobPrototype {
      name: self.name,
      server_type: self.server_type,
      lifetime: self.lifetime,
      effects,
    })
  }
}
