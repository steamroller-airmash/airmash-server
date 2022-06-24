//!

mod common;
mod config;
mod effect;
mod error;
mod game;
mod missile;
mod mob;
mod plane;
mod powerup;
mod special;
mod util;

#[cfg(feature = "script")]
mod script;

use std::borrow::Cow;
use std::fmt::Debug;

pub use self::common::GameConfigCommon;
pub use self::config::GameConfig;
pub use self::effect::EffectPrototype;
pub use self::error::{Path, Segment, ValidationError};
pub use self::game::GamePrototype;
pub use self::missile::MissilePrototype;
pub use self::mob::MobPrototype;
pub use self::plane::PlanePrototype;
pub use self::powerup::PowerupPrototype;
pub use self::special::*;

pub type Vector2 = nalgebra::Vector2<f32>;

mod sealed {
  pub trait Sealed {}
}

use self::sealed::Sealed;

pub trait PrototypeRef<'a>: Sealed {
  // Any traits we want to have automatically derived on the prototypes must be
  // mirrored here and the concrete instantiations must also derive them.
  type MissileRef: Clone + Debug + 'a;
  type SpecialRef: Clone + Debug + 'a;
  type PlaneRef: Clone + Debug + 'a;
  type MobRef: Clone + Debug + 'a;
  type PowerupRef: Clone + Debug + 'a;
}

#[derive(Copy, Clone, Debug)]
pub enum StringRef {}

#[derive(Copy, Clone, Debug)]
pub enum PtrRef {}

impl Sealed for StringRef {}
impl Sealed for PtrRef {}

impl<'a> PrototypeRef<'a> for StringRef {
  type MissileRef = Cow<'a, str>;
  type SpecialRef = Cow<'a, str>;
  type PowerupRef = Cow<'a, str>;
  type PlaneRef = Cow<'a, str>;
  type MobRef = Cow<'a, str>;
}

impl<'a> PrototypeRef<'a> for PtrRef {
  type MissileRef = &'a MissilePrototype;
  type SpecialRef = &'a SpecialPrototype<'a, Self>;
  type PowerupRef = &'a PowerupPrototype;
  type PlaneRef = &'a PlanePrototype<'a, Self>;
  type MobRef = &'a MobPrototype<'a, Self>;
}
