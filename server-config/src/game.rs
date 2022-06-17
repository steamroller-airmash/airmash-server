use std::ops::{Deref, DerefMut};

use crate::{GameConfigCommon, MissilePrototype, PlanePrototype, SpecialPrototype};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
pub struct GamePrototype {
  pub planes: Vec<PlanePrototype>,
  pub missiles: Vec<MissilePrototype>,
  pub specials: Vec<SpecialPrototype>,

  #[serde(flatten)]
  pub common: GameConfigCommon,
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
        SpecialPrototype::boost(),
        SpecialPrototype::multishot(),
        SpecialPrototype::strafe(),
        SpecialPrototype::repel(),
        SpecialPrototype::stealth(),
      ],
      common: GameConfigCommon::default(),
    }
  }
}

impl Deref for GamePrototype {
  type Target = GameConfigCommon;

  fn deref(&self) -> &Self::Target {
    &self.common
  }
}

impl DerefMut for GamePrototype {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.common
  }
}
