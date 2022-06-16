use crate::util::duration;
use std::borrow::Cow;
use std::time::Duration;

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
pub struct MultishotPrototype {
  /// The name of the missile prototype corresponding to the missile that will
  /// be fired.
  pub missile: Cow<'static, str>,

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
pub struct SpecialPrototype {
  /// The name with which this special effect will be referred to.
  pub name: Cow<'static, str>,
  /// Parameters for the general class of special that is being configured.
  #[serde(flatten)]
  pub data: SpecialPrototypeData,
}

/// Prototype for the special action of a plane
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum SpecialPrototypeData {
  /// No special effect whatsoever.
  #[serde(rename = "none")]
  None,
  #[serde(rename = "boost")]
  Boost(BoostPrototype),
  #[serde(rename = "multishot")]
  Multishot(MultishotPrototype),
  #[serde(rename = "repel")]
  Repel(RepelPrototype),
  #[serde(rename = "strafe")]
  Strafe,
  #[serde(rename = "stealth")]
  Stealth(StealthPrototype),
}

impl SpecialPrototype {
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

impl SpecialPrototype {
  pub const fn is_none(&self) -> bool {
    match self.data {
      SpecialPrototypeData::None => true,
      _ => false,
    }
  }

  pub const fn is_boost(&self) -> bool {
    match self.data {
      SpecialPrototypeData::Boost(_) => true,
      _ => false,
    }
  }

  pub const fn is_multishot(&self) -> bool {
    match self.data {
      SpecialPrototypeData::Multishot(_) => true,
      _ => false,
    }
  }

  pub const fn is_repel(&self) -> bool {
    match self.data {
      SpecialPrototypeData::Repel(_) => true,
      _ => false,
    }
  }

  pub const fn is_strafe(&self) -> bool {
    match self.data {
      SpecialPrototypeData::Strafe => true,
      _ => false,
    }
  }

  pub const fn is_stealth(&self) -> bool {
    match self.data {
      SpecialPrototypeData::Stealth(_) => true,
      _ => false,
    }
  }
}
