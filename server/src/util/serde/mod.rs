pub(crate) mod duration {
  use std::time::Duration;

  use serde::{Deserialize, Deserializer, Serializer};

  pub(crate) fn serialize<S: Serializer>(dur: &Duration, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_f64(dur.as_secs_f64())
  }

  pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Duration, D::Error> {
    f64::deserialize(de).map(Duration::from_secs_f64)
  }
}

pub(crate) mod option_duration {
  use std::time::Duration;

  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  pub(crate) fn serialize<S: Serializer>(
    dur: &Option<Duration>,
    ser: S,
  ) -> Result<S::Ok, S::Error> {
    dur.map(|d| d.as_secs_f64()).serialize(ser)
  }

  pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    de: D,
  ) -> Result<Option<Duration>, D::Error> {
    Ok(Option::deserialize(de)?.map(Duration::from_secs_f64))
  }
}

pub(crate) mod vector {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  use crate::Vector2;

  pub(crate) fn serialize<S: Serializer>(v: &Vector2, ser: S) -> Result<S::Ok, S::Error> {
    [v.x, v.y].serialize(ser)
  }

  pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Vector2, D::Error> {
    <[f32; 2]>::deserialize(de).map(From::from)
  }
}
