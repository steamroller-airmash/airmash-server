//!

#[macro_use]
extern crate serde;

mod missile;
mod plane;
mod special;
mod util;

pub use self::missile::MissilePrototype;
pub use self::plane::PlanePrototype;
pub use self::special::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct GamePrototype {
  pub planes: Vec<PlanePrototype>,
  pub missiles: Vec<MissilePrototype>,
  pub specials: Vec<SpecialPrototype>,
}

impl Default for GamePrototype {
  fn default() -> Self {
    Self {
      planes: vec![
        PlanePrototype::predator(),
        PlanePrototype::tornado(),
        PlanePrototype::mohawk(),
        PlanePrototype::goliath(),
        PlanePrototype::prowler(),
      ],
      missiles: vec![
        MissilePrototype::predator(),
        MissilePrototype::tornado(),
        MissilePrototype::tornado_triple(),
        MissilePrototype::prowler(),
        MissilePrototype::goliath(),
        MissilePrototype::mohawk(),
      ],
      specials: vec![
        SpecialPrototype::predator(),
        SpecialPrototype::tornado(),
        SpecialPrototype::mohawk(),
        SpecialPrototype::goliath(),
        SpecialPrototype::prowler(),
      ],
    }
  }
}
