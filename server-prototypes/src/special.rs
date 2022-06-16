use crate::util::duration;
use std::borrow::Cow;
use std::time::Duration;

/// Prototype for a boost effect similar to the predator boost.
///
/// # Example
/// The prototype for the predator boost looks like this:
/// ```
/// # use server::prototype::BoostPrototype;
/// # use std::borrow::Cow;
/// BoostPrototype {
///   name: Cow::Borrowed("boost"),
///   cost: 0.01,
///   speedup: 1.5,
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SpecialPrototypeData {
  /// No special effect whatsoever.
  None,
  Boost(BoostPrototype),
  Multishot(MultishotPrototype),
  Repel(RepelPrototype),
  Strafe,
  Stealth(StealthPrototype),
}

impl SpecialPrototype {
  pub const fn predator() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("boost"),
      data: Boost(BoostPrototype {
        cost: 0.01,
        speedup: 1.5,
      }),
    }
  }

  pub const fn tornado() -> Self {
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

  pub const fn goliath() -> Self {
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

  pub const fn mohawk() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("strafe"),
      data: Strafe,
    }
  }

  pub const fn prowler() -> Self {
    use self::SpecialPrototypeData::*;
    Self {
      name: Cow::Borrowed("prowler"),
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

use crate::util::TagSerializer;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl SpecialPrototypeData {
  const NAME: &'static str = "SpecialPrototypeData";
  const TAG: &'static str = "type";

  fn serialize_unit_variant<S>(&self, variant: &str, ser: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    use serde::ser::SerializeStruct as _;

    let mut ser = ser.serialize_struct(Self::NAME, 1)?;
    ser.serialize_field(Self::TAG, variant)?;
    ser.end()
  }

  fn serialize_variant<S, V>(&self, name: &str, proto: V, ser: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    V: Serialize,
  {
    proto.serialize(TagSerializer::new(Self::NAME, Self::TAG, name, ser))
  }
}

impl Serialize for SpecialPrototypeData {
  fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::None => self.serialize_unit_variant("none", ser),
      Self::Boost(proto) => self.serialize_variant("boost", proto, ser),
      Self::Multishot(proto) => self.serialize_variant("multishot", proto, ser),
      Self::Repel(proto) => self.serialize_variant("repel", proto, ser),
      Self::Strafe => self.serialize_unit_variant("strafe", ser),
      Self::Stealth(proto) => self.serialize_variant("stealth", proto, ser),
    }
  }
}

impl<'de> Deserialize<'de> for SpecialPrototypeData {
  fn deserialize<D>(de: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::{self, Visitor};

    struct DeVisitor;

    impl DeVisitor {
      const VARIANTS: &'static [&'static str] =
        &["none", "boost", "multishot", "repel", "strafe", "stealth"];

      fn deserialize_seq_variant<'de, V, A>(seq: A) -> Result<V, A::Error>
      where
        V: Deserialize<'de>,
        A: serde::de::SeqAccess<'de>,
      {
        use crate::util::SeqFwdDeserializer;

        V::deserialize(SeqFwdDeserializer(seq))
      }
    }

    impl<'de> Visitor<'de> for DeVisitor {
      type Value = SpecialPrototypeData;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("tagged result")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
      where
        A: serde::de::SeqAccess<'de>,
      {
        let tag: &'de str = match seq.next_element()? {
          Some(tag) => tag,
          None => return Err(de::Error::missing_field(SpecialPrototypeData::TAG)),
        };

        Ok(match tag {
          "none" => SpecialPrototypeData::None,
          "boost" => SpecialPrototypeData::Boost(Self::deserialize_seq_variant(seq)?),
          "multishot" => SpecialPrototypeData::Multishot(Self::deserialize_seq_variant(seq)?),
          "repel" => SpecialPrototypeData::Repel(Self::deserialize_seq_variant(seq)?),
          "strafe" => SpecialPrototypeData::Strafe,
          "stealth" => SpecialPrototypeData::Stealth(Self::deserialize_seq_variant(seq)?),
          _ => return Err(de::Error::unknown_variant(tag, Self::VARIANTS)),
        })
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: de::MapAccess<'de>,
      {
        use serde_value::{Value, ValueDeserializer};
        use std::collections::BTreeMap;

        let mut tag: Option<&'de str> = None;
        let mut entries: BTreeMap<Value, Value> = BTreeMap::new();

        while let Some(key) = map.next_key::<&'de str>()? {
          if key == SpecialPrototypeData::TAG {
            if tag.is_some() {
              return Err(de::Error::duplicate_field(SpecialPrototypeData::TAG));
            }

            tag = Some(map.next_value()?);
            continue;
          }

          if entries
            .insert(Value::String(key.to_owned()), map.next_value()?)
            .is_some()
          {
            return Err(de::Error::custom(format_args!("duplicate field `{}`", key)));
          }
        }

        let tag = match tag {
          Some(tag) => tag,
          None => return Err(de::Error::missing_field(SpecialPrototypeData::TAG)),
        };
        let de = ValueDeserializer::new(Value::Map(entries));

        Ok(match tag {
          "none" => SpecialPrototypeData::None,
          "boost" => SpecialPrototypeData::Boost(Deserialize::deserialize(de)?),
          "multishot" => SpecialPrototypeData::Multishot(Deserialize::deserialize(de)?),
          "repel" => SpecialPrototypeData::Repel(Deserialize::deserialize(de)?),
          "strafe" => SpecialPrototypeData::Strafe,
          "stealth" => SpecialPrototypeData::Stealth(Deserialize::deserialize(de)?),
          _ => return Err(de::Error::unknown_variant(tag, Self::FIELDS)),
        })
      }
    }

    de.deserialize_struct(Self::NAME, DeVisitor::VARIANTS, DeVisitor)
  }
}
