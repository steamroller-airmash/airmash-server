use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use crate::{
  GameConfigCommon, GamePrototype, MissilePrototype, PlanePrototype, PtrRef, SpecialPrototype,
  StringRef, ValidationError, ValidationExt,
};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GameConfig {
  pub planes: HashMap<&'static str, &'static PlanePrototype<'static, PtrRef>>,
  pub missiles: HashMap<&'static str, &'static MissilePrototype>,
  pub specials: HashMap<&'static str, &'static SpecialPrototype<'static, PtrRef>>,

  pub common: GameConfigCommon<'static, PtrRef>,

  data: GameConfigData,
}

impl GameConfig {
  /// Create a `GameConfig` from the provided [`GamePrototype`].
  pub fn new(proto: GamePrototype<'_, StringRef>) -> Result<Self, ValidationError> {
    // NOTE: If an error occurs then this function will leak memory. It's possible
    //       to fix this but there isn't currently a use case where this matters. If
    //       one comes up, then we'll fix it but otherwise it's cleaner to do it
    //       this way.

    let missiles = ManuallyDrop::new(
      proto
        .missiles
        .into_iter()
        .enumerate()
        .map(|(idx, m)| m.resolve().with(idx))
        .collect::<Result<Vec<_>, _>>()
        .with("missiles")?
        .into_boxed_slice(),
    );
    let specials = ManuallyDrop::new(
      proto
        .specials
        .into_iter()
        .enumerate()
        .map(|(idx, p)| p.resolve(&missiles).with(idx))
        .collect::<Result<Vec<_>, _>>()
        .with("specials")?
        .into_boxed_slice(),
    );
    let planes = ManuallyDrop::new(
      proto
        .planes
        .into_iter()
        .enumerate()
        .map(|(idx, p)| p.resolve(&missiles, &specials).with(idx))
        .collect::<Result<Vec<_>, _>>()
        .with("planes")?
        .into_boxed_slice(),
    );

    let data = unsafe { GameConfigData::new(&planes, &missiles, &specials) };

    let mut missiles = HashMap::new();
    let mut planes = HashMap::new();
    let mut specials = HashMap::new();

    for missile in data.missiles() {
      if missiles.insert(&*missile.name, missile).is_some() {
        return Err(
          ValidationError::custom("name", "multiple missile prototypes had the same name")
            .with(missile.name.to_string())
            .with("missiles"),
        );
      }
    }

    for special in data.specials() {
      if specials.insert(&*special.name, special).is_some() {
        return Err(
          ValidationError::custom("name", "multiple special prototypes had the same name")
            .with(special.name.to_string())
            .with("specials"),
        );
      }
    }

    for plane in data.planes() {
      if planes.insert(&*plane.name, plane).is_some() {
        return Err(
          ValidationError::custom("name", "multiple plane prototypes had the same name")
            .with(plane.name.to_string())
            .with("specials"),
        );
      }
    }

    Ok(Self {
      missiles,
      planes,
      specials,

      common: proto.common.resolve(data.planes())?,
      data,
    })
  }

  fn into_data(self) -> GameConfigData {
    self.data
  }

  /// Unsafelly reclaim and free a static reference that was created by calling
  /// `leak`. This is mainly useful for tracking leaks in other parts of the
  /// program.
  ///
  /// # Safety
  /// - The reference to `self` must never be used again after this method is
  ///   called.
  pub unsafe fn reclaim(self) {
    self.into_data().reclaim();
  }
}

impl Default for GameConfig {
  fn default() -> Self {
    Self::new(GamePrototype::default()).unwrap()
  }
}

impl TryFrom<GamePrototype<'_, StringRef>> for GameConfig {
  type Error = ValidationError;

  fn try_from(value: GamePrototype<'_, StringRef>) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}

impl Deref for GameConfig {
  type Target = GameConfigCommon<'static, PtrRef>;

  fn deref(&self) -> &Self::Target {
    &self.common
  }
}

impl DerefMut for GameConfig {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.common
  }
}

#[derive(Clone)]
struct GameConfigData {
  planes: NonNull<[PlanePrototype<'static, PtrRef>]>,
  missiles: NonNull<[MissilePrototype]>,
  specials: NonNull<[SpecialPrototype<'static, PtrRef>]>,
}

impl GameConfigData {
  /// Create a set of GameConfigData.
  ///
  /// # Safety
  /// None of the prototypes may refer to any non-static data outside of that
  /// being passed in here. The slices must not be dropped except by safely
  /// calling [`reclaim`] once it is safe to do so.
  unsafe fn new(
    planes: &[PlanePrototype<PtrRef>],
    missiles: &[MissilePrototype],
    specials: &[SpecialPrototype<PtrRef>],
  ) -> Self {
    Self {
      planes: NonNull::new(planes as *const _ as *mut _).unwrap(),
      missiles: NonNull::new(missiles as *const _ as *mut _).unwrap(),
      specials: NonNull::new(specials as *const _ as *mut _).unwrap(),
    }
  }

  /// Free all the memory that had been previously leaked.
  ///
  /// # Safety
  /// There must be no existing references to any of the data stored within this
  /// type or else those references will be left as dangling references.
  unsafe fn reclaim(self) {
    // Note: Order matters here!
    let _ = Box::from_raw(self.planes.as_ptr());
    let _ = Box::from_raw(self.specials.as_ptr());
    let _ = Box::from_raw(self.missiles.as_ptr());
  }

  fn planes(&self) -> &'static [PlanePrototype<'static, PtrRef>] {
    unsafe { self.planes.as_ref() }
  }

  fn missiles(&self) -> &'static [MissilePrototype] {
    unsafe { self.missiles.as_ref() }
  }

  fn specials(&self) -> &'static [SpecialPrototype<'static, PtrRef>] {
    unsafe { self.specials.as_ref() }
  }
}

impl fmt::Debug for GameConfigData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("..")
  }
}
