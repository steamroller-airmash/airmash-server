//!

#[macro_use]
extern crate serde;

mod common;
mod config;
mod error;
mod game;
mod missile;
mod mob;
mod plane;
mod special;
mod util;

#[cfg(feature = "script")]
mod script;

use std::borrow::Cow;
use std::fmt::Debug;

pub use self::common::GameConfigCommon;
pub use self::config::GameConfig;
pub(crate) use self::error::ValidationExt;
pub use self::error::{Path, Segment, ValidationError};
pub use self::game::GamePrototype;
pub use self::missile::MissilePrototype;
pub use self::mob::MobPrototype;
pub use self::plane::PlanePrototype;
pub use self::special::*;

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
  type PlaneRef = Cow<'a, str>;
  type MobRef = Cow<'a, str>;
}

impl<'a> PrototypeRef<'a> for PtrRef {
  type MissileRef = &'a MissilePrototype;
  type SpecialRef = &'a SpecialPrototype<'a, Self>;
  type PlaneRef = &'a PlanePrototype<'a, Self>;
  type MobRef = &'a MobPrototype;
}
