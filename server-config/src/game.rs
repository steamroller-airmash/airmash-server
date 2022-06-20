use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
  GameConfigCommon, MissilePrototype, MobPrototype, PlanePrototype, PrototypeRef, SpecialPrototype,
  StringRef,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
#[serde(bound(
  serialize = "
    Ref::MissileRef: Serialize,
    Ref::SpecialRef: Serialize,
    Ref::PlaneRef: Serialize,
    Ref::MobRef: Serialize,
  ",
  deserialize = "
    Ref::MissileRef: Deserialize<'de>,
    Ref::SpecialRef: Deserialize<'de>,
    Ref::PlaneRef: Deserialize<'de>,
    Ref::MobRef: Deserialize<'de>,
  "
))]
pub struct GamePrototype<'a, Ref: PrototypeRef<'a>> {
  pub planes: Vec<PlanePrototype<'a, Ref>>,
  pub missiles: Vec<MissilePrototype>,
  pub specials: Vec<SpecialPrototype<'a, Ref>>,
  pub mobs: Vec<MobPrototype>,

  #[serde(flatten)]
  pub common: GameConfigCommon<'a, Ref>,
}

impl Default for GamePrototype<'_, StringRef> {
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
      mobs: vec![
        MobPrototype::inferno(),
        MobPrototype::shield(),
        MobPrototype::upgrade(),
      ],
      common: GameConfigCommon::default(),
    }
  }
}

impl<'a, R: PrototypeRef<'a>> Deref for GamePrototype<'a, R> {
  type Target = GameConfigCommon<'a, R>;

  fn deref(&self) -> &Self::Target {
    &self.common
  }
}

impl<'a, R: PrototypeRef<'a>> DerefMut for GamePrototype<'a, R> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.common
  }
}
