use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use crate::util::{DropPtr, MaybeDrop};
use crate::{
  GameConfigCommon, GamePrototype, MissilePrototype, MobPrototype, PlanePrototype,
  PowerupPrototype, PtrRef, SpecialPrototype, StringRef, ValidationError,
};

macro_rules! transform_protos {
  ($proto:expr => |$item:ident| $resolved:expr) => {{
    let iter = $proto.into_iter().enumerate();
    let mut vals = Vec::with_capacity(iter.size_hint().1.unwrap_or(0));
    let mut result: Option<ValidationError> = None;

    for (idx, $item) in iter {
      match $resolved {
        Ok(val) => vals.push(val),
        Err(e) => {
          result = Some(e.with(idx));
          break;
        }
      }
    }

    match result {
      Some(err) => Err(err),
      None => Ok(ManuallyDrop::new(vals.into_boxed_slice())),
    }
  }};
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GameConfig {
  pub planes: HashMap<&'static str, &'static PlanePrototype<'static, PtrRef>>,
  pub missiles: HashMap<&'static str, &'static MissilePrototype>,
  pub specials: HashMap<&'static str, &'static SpecialPrototype<'static, PtrRef>>,
  pub powerups: HashMap<&'static str, &'static PowerupPrototype>,
  pub mobs: HashMap<&'static str, &'static MobPrototype<'static, PtrRef>>,

  pub common: GameConfigCommon<'static, PtrRef>,

  data: GameConfigData,
}

impl GameConfig {
  fn new_internal(
    planes: &[PlanePrototype<PtrRef>],
    missiles: &[MissilePrototype],
    specials: &[SpecialPrototype<PtrRef>],
    mobs: &[MobPrototype<PtrRef>],
    powerups: &[PowerupPrototype],

    common: GameConfigCommon<StringRef>,
  ) -> Result<Self, ValidationError> {
    let data = unsafe { GameConfigData::new(&planes, &missiles, &specials, &mobs, &powerups) };

    let mut missiles = HashMap::new();
    let mut planes = HashMap::new();
    let mut specials = HashMap::new();
    let mut effects = HashMap::new();
    let mut mobs = HashMap::new();

    for effect in data.powerups() {
      if effects.insert(&*effect.name, effect).is_some() {
        return Err(
          ValidationError::custom("name", "multiple effect prototypes had the same name")
            .with(effect.name.to_string())
            .with("mobs"),
        );
      }
    }

    for mob in data.mobs() {
      if mobs.insert(&*mob.name, mob).is_some() {
        return Err(
          ValidationError::custom("name", "multiple mob prototypes had the same name")
            .with(mob.name.to_string())
            .with("mobs"),
        );
      }
    }

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
      powerups: effects,
      mobs,

      common: common.resolve(data.planes())?,
      data,
    })
  }

  /// Create a `GameConfig` from the provided [`GamePrototype`].
  pub fn new(proto: GamePrototype<'_, StringRef>) -> Result<Self, ValidationError> {
    // These ones will be automatically dropped if something goes wrong. In order to
    // prevent UB we just need to call MaybeDrop::cancel_drop if everything works
    // out at the end.
    let missiles = MaybeDrop::from(transform_protos!(proto.missiles => |m| m.resolve())?);
    let effects = MaybeDrop::from(transform_protos!(proto.powerups => |m| m.resolve())?);
    let mobs = MaybeDrop::from(transform_protos!(proto.mobs => |m| m.resolve(&effects))?);

    let mut specials = transform_protos!(proto.specials => |s| s.resolve(&missiles))?;
    // Due to some lifetime issues it's not actually possible to store specials in
    // something that has a drop function (it would need to be dropped at the exact
    // same time as `missiles` due to the lifetimes in PlanePrototype::resolve being
    // invariant). However, to make this all work we can (unsafely) create a second
    // struct that will drop it at the correct time and which we can cancel the drop
    // if everything goes successfully.
    //
    // SAFETY: special_dropper is dropped just before specials would be dropped so
    //         it is safe to have it drop specials instead of having a MaybeDrop
    //         instance directly wrapping specials.
    let special_dropper = MaybeDrop::new(unsafe { DropPtr::new(&mut specials) });

    let planes =
      MaybeDrop::from(transform_protos!(proto.planes => |p| p.resolve(&missiles, &specials))?);

    let config = Self::new_internal(&planes, &missiles, &specials, &mobs, &effects, proto.common)?;

    MaybeDrop::cancel_drop(&mobs);
    MaybeDrop::cancel_drop(&missiles);
    MaybeDrop::cancel_drop(&effects);
    MaybeDrop::cancel_drop(&special_dropper);
    MaybeDrop::cancel_drop(&planes);

    Ok(config)
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
  effects: NonNull<[PowerupPrototype]>,
  mobs: NonNull<[MobPrototype<'static, PtrRef>]>,
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
    mobs: &[MobPrototype<PtrRef>],
    effects: &[PowerupPrototype],
  ) -> Self {
    Self {
      planes: NonNull::new(planes as *const _ as *mut _).unwrap(),
      missiles: NonNull::new(missiles as *const _ as *mut _).unwrap(),
      specials: NonNull::new(specials as *const _ as *mut _).unwrap(),
      effects: NonNull::new(effects as *const _ as *mut _).unwrap(),
      mobs: NonNull::new(mobs as *const _ as *mut _).unwrap(),
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
    let _ = Box::from_raw(self.mobs.as_ptr());
    let _ = Box::from_raw(self.effects.as_ptr());
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

  fn mobs(&self) -> &'static [MobPrototype<'static, PtrRef>] {
    unsafe { self.mobs.as_ref() }
  }

  fn powerups(&self) -> &'static [PowerupPrototype] {
    unsafe { self.effects.as_ref() }
  }
}

impl fmt::Debug for GameConfigData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("..")
  }
}
