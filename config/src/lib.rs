//!

use fxhash::FxHashMap;
use std::borrow::Cow;

mod effect;
mod missile;
mod mob;
mod plane;

pub use self::missile::MissileProto;
pub use self::mob::MobProto;
pub use self::plane::PlaneProto;

type CowString = Cow<'static, str>;

pub trait Prototype {
  type Concrete;

  fn resolve(&self, proto: &'static GameProto) -> Self::Concrete;
}

pub struct GameProto {
  pub missiles: FxHashMap<CowString, MissileProto>,
  pub planes: FxHashMap<CowString, PlaneProto>,
  pub mobs: FxHashMap<CowString, MobProto>,
}
