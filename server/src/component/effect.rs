use std::collections::HashMap;
use std::time::Instant;

use crate::config::EffectPrototypeRef;
use crate::protocol::PowerupType;

/// Effect manager for a player.
///
/// This component tracks the set of effects that a player has. It has two types
/// of effects:
/// 1. short-term effects associated with a powerup, and,
/// 2. long-term effects that have their lifetime explicitly managed.
#[derive(Clone, Debug, Default)]
pub struct Effects {
  permanent: HashMap<&'static str, EffectPrototypeRef>,
  powerup: Option<PowerupEffects>,
}

#[derive(Clone, Debug)]
struct PowerupEffects {
  powerup: PowerupType,
  expiry: Instant,
  effects: &'static [EffectPrototypeRef],
}

impl Effects {
  /// Enable a new set of effects associated with the powerup. This will
  /// overwrite any effects associated with the previously active powerup.
  pub fn set_powerup(
    &mut self,
    powerup: PowerupType,
    expiry: Instant,
    effects: &'static [EffectPrototypeRef],
  ) {
    self.powerup = Some(PowerupEffects {
      powerup,
      expiry,
      effects,
    });
  }

  pub fn clear_powerup(&mut self) {
    self.powerup = None;
  }

  /// Get the expiry time of the current powerup.
  pub fn expiry(&self) -> Option<Instant> {
    self.powerup.as_ref().map(|p| p.expiry)
  }

  /// Get the server type of the current powerup.
  pub fn powerup(&self) -> Option<PowerupType> {
    self.powerup.as_ref().map(|p| p.powerup)
  }

  /// Add a new long-term effect. Long-term effects are deduplicated by name.
  pub fn add_effect(&mut self, effect: EffectPrototypeRef) {
    self.permanent.insert(&*effect.name, effect);
  }

  /// Remove a long-term effect by prototype name.
  pub fn erase_effect(&mut self, name: &str) -> bool {
    self.permanent.remove(name).is_some()
  }

  pub fn effects(&self) -> impl Iterator<Item = EffectPrototypeRef> + '_ {
    let permanent = self.permanent.iter().map(|x| *x.1);

    let temporary = self
      .powerup
      .as_ref()
      .map(|p| p.effects)
      .unwrap_or(&[])
      .into_iter()
      .copied();

    permanent.chain(temporary)
  }
}
