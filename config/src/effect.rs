use crate::CowString;
use std::{borrow::Cow, time::Duration};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServerEffect {
  Shield,
  Inferno,
  None,
}

#[derive(Clone, Debug)]
pub struct EffectProto {
  /// The name of this effect.
  pub name: CowString,
  /// The type of this effect as communicated to the server.
  pub server_type: ServerEffect,
  ///
  pub duration: Option<Duration>,
  pub effect: EffectData,
}

#[derive(Clone, Debug)]
pub enum EffectData {
  /// Multiplies any damage recieved by the player by the provided multiplier.
  DamageResistance(f64),
  /// Allows the player to shoot more missiles while the effect is active.
  Inferno,
  /// Grant a single unused upgrade to the player.
  Upgrade,
  /// Override the maximum speed of the player.
  FixSpeed(f64),
}

impl EffectProto {
  pub const fn shield() -> Self {
    Self {
      name: Cow::Borrowed("shield"),
      server_type: ServerEffect::Shield,
      duration: Some(Duration::from_secs(10)),
      effect: EffectData::DamageResistance(0.0),
    }
  }

  pub fn spawn_shield() -> Self {
    Self {
      name: Cow::Borrowed("spawn_shield"),
      duration: Some(Duration::from_secs(2)),
      ..Self::shield()
    }
  }

  pub const fn inferno() -> Self {
    Self {
      name: Cow::Borrowed("inferno"),
      server_type: ServerEffect::Inferno,
      duration: Some(Duration::from_secs(10)),
      effect: EffectData::Inferno,
    }
  }

  pub const fn flag_speed() -> Self {
    Self {
      name: Cow::Borrowed("flag_speed"),
      server_type: ServerEffect::None,
      duration: None,
      effect: EffectData::FixSpeed(5.0),
    }
  }
}
