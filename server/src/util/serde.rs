pub(crate) mod duration {
  use serde::{Deserialize, Deserializer, Serializer};
  use std::time::Duration;

  pub(crate) fn serialize<S: Serializer>(dur: &Duration, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_f64(dur.as_secs_f64())
  }

  pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Duration, D::Error> {
    f64::deserialize(de).map(Duration::from_secs_f64)
  }
}

pub(crate) mod option_duration {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use std::time::Duration;

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
