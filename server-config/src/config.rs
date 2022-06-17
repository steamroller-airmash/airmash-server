use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::{
  GameConfigCommon, GamePrototype, MissilePrototype, PlanePrototype, SpecialPrototype,
  SpecialPrototypeData, ValidationError, ValidationExt,
};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GameConfig {
  pub planes: HashMap<String, PlanePrototype>,
  pub missiles: HashMap<String, MissilePrototype>,
  pub specials: HashMap<String, SpecialPrototype>,

  pub common: GameConfigCommon,
}

impl GameConfig {
  /// Create a `GameConfig` from the provided [`GamePrototype`].
  pub fn new(proto: GamePrototype) -> Result<Self, ValidationError> {
    let mut missiles = HashMap::new();
    let mut planes = HashMap::new();
    let mut specials = HashMap::new();

    for (idx, missile) in proto.missiles.into_iter().enumerate() {
      if missile.name.is_empty() {
        return Err(
          ValidationError::custom("name", "missile prototype had empty name")
            .with(idx)
            .with("missiles"),
        );
      }

      if let Some(missile) = missiles.insert(missile.name.to_string(), missile) {
        return Err(
          ValidationError::custom("name", "multiple missile prototypes had the same name")
            .with(missile.name.into_owned())
            .with("missiles"),
        );
      }
    }

    for (idx, special) in proto.specials.into_iter().enumerate() {
      if special.name.is_empty() {
        return Err(
          ValidationError::custom("name", "special prototype had empty name")
            .with(idx)
            .with("specials"),
        );
      }

      if let SpecialPrototypeData::Multishot(multishot) = &special.data {
        if !missiles.contains_key(&*multishot.missile) {
          return Err(ValidationError::custom(
            "missile",
            format_args!(
              "multishot special refers to nonexistant missile prototype `{}`",
              multishot.missile
            ),
          ))
          .with(special.name.into_owned())
          .with("specials");
        }
      }

      if let Some(special) = specials.insert(special.name.to_string(), special) {
        return Err(
          ValidationError::custom("name", "multiple missile prototypes had the same name")
            .with(special.name.into_owned())
            .with("specials"),
        );
      }
    }

    for (idx, plane) in proto.planes.into_iter().enumerate() {
      if plane.name.is_empty() {
        return Err(
          ValidationError::custom("name", "plane prototype had empty name")
            .with(idx)
            .with("planes"),
        );
      }

      if !missiles.contains_key(&*plane.missile) {
        return Err(
          ValidationError::custom(
            "missile",
            format_args!(
              "plane prototype refers to a nonexistant missile prototype `{}`",
              plane.missile
            ),
          )
          .with(plane.name.into_owned())
          .with("planes"),
        );
      }

      if !specials.contains_key(&*plane.special) {
        return Err(
          ValidationError::custom(
            "special",
            format_args!(
              "plane prototype refers to nonexistant special prototype `{}`",
              plane.special
            ),
          )
          .with(plane.name.into_owned())
          .with("planes"),
        );
      }

      if let Some(plane) = planes.insert(plane.name.to_string(), plane) {
        return Err(
          ValidationError::custom("name", "multiple missile prototypes had the same name")
            .with(plane.name.into_owned())
            .with("plane"),
        );
      }
    }

    if !planes.contains_key(&*proto.common.default_plane) {
      return Err(ValidationError::custom(
        "default_plane",
        format_args!(
          "default_plane refers to a plane prototype `{}` which does not exist",
          proto.common.default_plane
        ),
      ));
    }

    Ok(Self {
      missiles,
      planes,
      specials,

      common: proto.common,
    })
  }

  /// Purposefully leak this `GameConfig` in order to allow for static
  /// references to be stored easily within the server datastructures.
  pub fn leak(self) -> &'static mut Self {
    Box::leak(Box::new(self))
  }

  /// Unsafelly reclaim and free a static reference that was created by calling
  /// `leak`. This is mainly useful for tracking leaks in other parts of the
  /// program.
  ///
  /// # Safety
  /// - The reference to `self` must never be used again after this method is
  ///   called.
  pub unsafe fn reclaim(&'static mut self) {
    let _ = Box::from_raw(self as *const Self as *mut Self);
  }
}

impl Default for GameConfig {
  fn default() -> Self {
    Self::new(GamePrototype::default()).unwrap()
  }
}

impl TryFrom<GamePrototype> for GameConfig {
  type Error = ValidationError;

  fn try_from(value: GamePrototype) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}

impl Deref for GameConfig {
  type Target = GameConfigCommon;

  fn deref(&self) -> &Self::Target {
    &self.common
  }
}

impl DerefMut for GameConfig {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.common
  }
}
