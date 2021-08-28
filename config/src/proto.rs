#![allow(dead_code)]

use protocol::{MobType, PlaneType};
use std::{borrow::Cow, time::Duration};


pub struct EffectProto {
  pub name: CowString,
}

pub struct MobProto {
  pub name: CowString,
  pub server_type: MobType,
}

