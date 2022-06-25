use std::cell::Cell;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

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

/// Wrapper type around [`ManuallyDrop`] which drops the contained value unless
/// it is explicitly prevented from doing so.
pub(crate) struct MaybeDrop<T> {
  item: ManuallyDrop<T>,
  flag: Cell<bool>,
}

impl<T> MaybeDrop<T> {
  pub fn new(item: T) -> Self {
    Self {
      item: ManuallyDrop::new(item),
      flag: Cell::new(true),
    }
  }

  /// Prevent the contained value from being dropped when this `MaybeDrop` is
  /// dropped.
  pub fn cancel_drop(slot: &Self) {
    slot.flag.set(false)
  }
}

impl<T> From<ManuallyDrop<T>> for MaybeDrop<T> {
  fn from(item: ManuallyDrop<T>) -> Self {
    Self::new(ManuallyDrop::into_inner(item))
  }
}

impl<T> Drop for MaybeDrop<T> {
  fn drop(&mut self) {
    if self.flag.get() {
      // SAFETY: This is the only place where self.item is dropped so there is no
      //         possibility of double-drops.
      unsafe { ManuallyDrop::drop(&mut self.item) }
    }
  }
}

impl<T> Deref for MaybeDrop<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.item
  }
}

impl<T> DerefMut for MaybeDrop<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.item
  }
}

/// RAII wrapper that drops whatever the stored pointer points to.
pub(crate) struct DropPtr<T>(*mut ManuallyDrop<T>);

impl<T> DropPtr<T> {
  /// # Safety
  /// `ptr` must be valid to drop until the `DropPtr` instance drops or is
  /// forgotten.
  pub unsafe fn new(ptr: *mut ManuallyDrop<T>) -> Self {
    Self(ptr)
  }
}

impl<T> Drop for DropPtr<T> {
  fn drop(&mut self) {
    // SAFETY: The safety contract for DropPtr::new guarantees that this is safe.
    unsafe { ManuallyDrop::drop(&mut *self.0) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct DropWrite<'a>(&'a mut bool);

  impl Drop for DropWrite<'_> {
    fn drop(&mut self) {
      *self.0 = true;
    }
  }

  #[test]
  fn maybedrop_drops_by_default() {
    let mut check = false;

    {
      let _drop = MaybeDrop::new(DropWrite(&mut check));
    }

    assert!(check);
  }

  #[test]
  fn maybedrop_no_drop_when_disabled() {
    let mut check = false;

    {
      let drop = MaybeDrop::new(DropWrite(&mut check));
      MaybeDrop::cancel_drop(&drop);
    }

    assert!(!check);
  }
}
