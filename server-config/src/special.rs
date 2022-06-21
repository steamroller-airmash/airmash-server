use std::borrow::Cow;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::util::duration;
use crate::{MissilePrototype, PrototypeRef, PtrRef, StringRef, ValidationError};

/// Prototype for a boost effect similar to the predator boost.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
pub struct BoostPrototype {
  /// The rate at which boosting uses up energy in (energy/frame)
  pub cost: f32,
  /// A multiplier that multiplies both the maximum speed and the accelerating
  /// while the plane is boosting.
  pub speedup: f32,
}

/// Prototype for a special which fires a number of missiles.
///
/// This is the same special effect as the tornado.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
#[serde(bound(
  serialize = "Ref::MissileRef: serde::Serialize",
  deserialize = "Ref::MissileRef: serde::Deserialize<'de>"
))]
pub struct MultishotPrototype<'a, Ref: PrototypeRef<'a>> {
  /// The missile prototype corresponding to the missile that will be fired.
  pub missile: Ref::MissileRef,

  /// The number of missiles that will be fired when this special is triggered.
  /// Note that if this is an even number then it will be rounded up to an odd
  /// number when firing.
  pub count: u8,

  /// The cost of firing the missiles.
  pub cost: f32,

  /// The minimum delay between successive firings.
  ///
  /// If this is 0 then the plane will be able to fire whenever it has
  /// sufficient energy to do so and will keep firing each frame until it no
  /// longer has the required energy. This is not usually what you want,
  /// although it may work fine if the energy cost is large enough to prevent
  /// repeated missile firings.
  #[serde(with = "duration")]
  pub delay: Duration,

  /// The X offset of the furthest missile away from the plane. (0, 0) would
  /// place the missile at the same place
  pub offset_x: f32,
  /// The Y offset of the furthest missile away from the plane.
  pub offset_y: f32,
}

/// Prototype for a goliath repel.
// TODO: We might want to have repels which only effect one of missiles and players instead of
//       always doing both.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
pub struct RepelPrototype {
  /// The range out to which missiles will be repelled when this special is
  /// triggered.
  pub range: f32,

  /// The energy cost of using repel.
  pub cost: f32,

  /// The minimum delay between successive repels. Even if the player has enough
  /// energy they will not be able to repel until at least this amount of time
  /// has passed.
  #[serde(with = "duration")]
  pub delay: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
pub struct StealthPrototype {
  /// The cost of engaging stealth. Dropping out of stealth is always free.
  pub cost: f32,

  /// The minimum duration between dropping out of stealth and being able to go
  /// back into stealth.
  #[serde(with = "duration")]
  pub delay: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
pub struct SpecialPrototype<'a, Ref: PrototypeRef<'a> = StringRef> {
  /// The name with which this special effect will be referred to.
  pub name: Cow<'static, str>,
  /// Parameters for the general class of special that is being configured.
  #[serde(flatten)]
  pub data: SpecialPrototypeData<'a, Ref>,
}

/// Prototype for the special action of a plane
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
#[serde(bound(
  serialize = "Ref::MissileRef: serde::Serialize",
  deserialize = "Ref::MissileRef: serde::Deserialize<'de>"
))]
pub enum SpecialPrototypeData<'a, Ref: PrototypeRef<'a>> {
  /// No special effect whatsoever.
  #[serde(rename = "none")]
  None,
  #[serde(rename = "boost")]
  Boost(BoostPrototype),
  #[serde(rename = "multishot")]
  Multishot(MultishotPrototype<'a, Ref>),
  #[serde(rename = "repel")]
  Repel(RepelPrototype),
  #[serde(rename = "strafe")]
  Strafe,
  #[serde(rename = "stealth")]
  Stealth(StealthPrototype),
}

impl SpecialPrototype<'_, StringRef> {
  pub const fn none() -> Self {
    Self {
      name: Cow::Borrowed("none"),
      data: SpecialPrototypeData::None,
    }
  }

  pub const fn boost() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("boost"),
      data: Boost(BoostPrototype {
        cost: 0.01,
        speedup: 1.5,
      }),
    }
  }

  pub const fn multishot() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("multishot"),
      data: Multishot(MultishotPrototype {
        missile: Cow::Borrowed("tornado-triple"),
        count: 3,
        cost: 0.9,

        // The cost of the default tornado special is high enough that it doesn't actually need a
        // delay to avoid being repeatedly fired. However, the defaults for a tornado are expected
        // to be reused for all the modified versions that people make. As such, I've chosen
        // 300ms here as a sane default of 0.5s since that matches the tornado firing delay.
        delay: Duration::from_millis(500),

        // TODO: This is wrong for tornado specials with an inferno. It will be necessary to figure
        //       out some sort of adjustment so that it matches up with the original game but still
        //       extends reasonably to an increased number of missiles.
        offset_x: 15.0,
        offset_y: 9.6,
      }),
    }
  }

  pub const fn repel() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("repel"),
      data: Repel(RepelPrototype {
        // Note: When the repel radius was measured on the original server we came up with two
        //       different radii for missile repels and player repels. I believe this was due to
        //       measurement error in how it was measured so I'm consolidating this as a single
        //       range. As a todo, this could be remeasured using congratulio's airmash
        //       captures to remeasure and find the real range.
        range: 225.0,
        cost: 0.5,
        delay: Duration::from_secs(1),
      }),
    }
  }

  pub const fn strafe() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("strafe"),
      data: Strafe,
    }
  }

  pub const fn stealth() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("stealth"),
      data: Stealth(StealthPrototype {
        cost: 0.6,
        delay: Duration::from_millis(1500),
      }),
    }
  }
}

impl<'a, R: PrototypeRef<'a>> SpecialPrototype<'a, R> {
  pub const fn as_boost(&self) -> Option<&BoostPrototype> {
    match &self.data {
      SpecialPrototypeData::Boost(boost) => Some(boost),
      _ => None,
    }
  }

  pub const fn as_multishot(&self) -> Option<&MultishotPrototype<'a, R>> {
    match &self.data {
      SpecialPrototypeData::Multishot(multishot) => Some(multishot),
      _ => None,
    }
  }

  pub const fn as_repel(&self) -> Option<&RepelPrototype> {
    match &self.data {
      SpecialPrototypeData::Repel(repel) => Some(repel),
      _ => None,
    }
  }

  pub const fn as_stealth(&self) -> Option<&StealthPrototype> {
    match &self.data {
      SpecialPrototypeData::Stealth(stealth) => Some(stealth),
      _ => None,
    }
  }

  pub const fn is_none(&self) -> bool {
    matches!(self.data, SpecialPrototypeData::None)
  }

  pub const fn is_boost(&self) -> bool {
    matches!(self.data, SpecialPrototypeData::Boost(_))
  }

  pub const fn is_multishot(&self) -> bool {
    matches!(self.data, SpecialPrototypeData::Multishot(_))
  }

  pub const fn is_repel(&self) -> bool {
    matches!(self.data, SpecialPrototypeData::Repel(_))
  }

  pub const fn is_strafe(&self) -> bool {
    matches!(self.data, SpecialPrototypeData::Strafe)
  }

  pub const fn is_stealth(&self) -> bool {
    matches!(self.data, SpecialPrototypeData::Stealth(_))
  }
}

impl MultishotPrototype<'_, StringRef> {
  pub(crate) fn resolve(
    self,
    missiles: &[MissilePrototype],
  ) -> Result<MultishotPrototype<PtrRef>, ValidationError> {
    let missile = missiles
      .iter()
      .find(|missile| missile.name == self.missile)
      .ok_or(ValidationError::custom(
        "missile",
        format_args!(
          "multishot special refers to nonexistant missile prototype `{}`",
          self.missile
        ),
      ))?;

    // FIXME: Once <https://github.com/rust-lang/rust/issues/86555> stabilizes we can replace this with
    //        Ok(MultishotPrototype { missile, ..self })
    Ok(MultishotPrototype {
      missile,
      count: self.count,
      cost: self.cost,
      delay: self.delay,
      offset_x: self.offset_x,
      offset_y: self.offset_y,
    })
  }
}

impl SpecialPrototype<'_, StringRef> {
  pub(crate) fn resolve<'a>(
    self,
    missiles: &'a [MissilePrototype],
  ) -> Result<SpecialPrototype<'a, PtrRef>, ValidationError> {
    use self::SpecialPrototypeData::*;

    if self.name.is_empty() {
      return Err(ValidationError::custom(
        "name",
        "special prototype had an empty name",
      ));
    }

    let data = match self.data {
      None => None,
      Strafe => Strafe,
      Boost(x) => Boost(x),
      Repel(x) => Repel(x),
      Stealth(x) => Stealth(x),
      Multishot(x) => Multishot(x.resolve(missiles)?),
    };

    Ok(SpecialPrototype {
      name: self.name,
      data,
    })
  }
}
