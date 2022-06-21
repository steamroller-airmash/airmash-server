use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename = "kebab-case")]
pub enum EffectPrototype {
  Shield {
    damage_mult: f32,
  },
  Inferno,
  FixedSpeed {
    speed: f32,
  },
  Upgrade,
  /// Despawn the mob that just collided.
  Despawn,
  /// Multiply the max speed of the plane.
  Speed {
    speed_mult: f32,
  },
}

impl EffectPrototype {
  pub const fn shield() -> Self {
    Self::Shield { damage_mult: 0.0 }
  }

  pub const fn inferno() -> Self {
    Self::Inferno
  }

  pub const fn flag_speed() -> Self {
    Self::FixedSpeed { speed: 5.0 }
  }

  pub const fn upgrade() -> Self {
    Self::Upgrade
  }

  pub const fn despawn() -> Self {
    Self::Despawn
  }
}

impl EffectPrototype {
  pub const fn is_shield(&self) -> bool {
    matches!(self, Self::Shield { .. })
  }

  pub const fn is_inferno(&self) -> bool {
    matches!(self, Self::Inferno)
  }

  pub const fn is_fixed_speed(&self) -> bool {
    matches!(self, Self::FixedSpeed { .. })
  }

  pub const fn is_upgrade(&self) -> bool {
    matches!(self, Self::Upgrade)
  }

  pub const fn is_despawn(&self) -> bool {
    matches!(self, Self::Despawn)
  }

  pub const fn is_speed(&self) -> bool {
    matches!(self, Self::Speed { .. })
  }

  pub const fn is_instant(&self) -> bool {
    matches!(self, Self::Upgrade | Self::Despawn)
  }
}
