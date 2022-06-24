//! Configuration info controlling how planes and missiles behave.

use std::ops::Index;
use std::time::Duration;

use nalgebra::vector;
use serde_deserialize_over::DeserializeOver;

use crate::protocol::{
  AccelScalar, Distance, Energy, EnergyRegen, Health, HealthRegen, MobType, MobType as Mob,
  PlaneType as Plane, Rotation, RotationRate, Speed,
};
use crate::util::serde::*;
use crate::Vector2;

#[derive(Debug, Clone, Serialize, Deserialize, DeserializeOver)]
pub struct PlaneInfo {
  // Rotation
  pub turn_factor: RotationRate,

  // Acceleration
  pub accel_factor: AccelScalar,
  pub brake_factor: AccelScalar,
  pub boost_factor: f32,

  // Speeds
  pub max_speed: Speed,
  pub min_speed: Speed,
  pub flag_speed: Speed,
  pub inferno_factor: f32,

  // Regen
  pub health_regen: HealthRegen,
  pub energy_regen: EnergyRegen,

  // Health
  pub damage_factor: f32,

  // Energy requirement
  pub fire_energy: Energy,
  #[serde(with = "duration")]
  pub fire_delay: Duration,

  // Type of missile that the plane fires
  pub missile_type: Mob,
  // Offset of missile relative to the plane when fired.
  //
  // The horizontal (Y) offset will alternate sides with every shot.
  pub missile_offset: Vector2<Distance>,

  // Angle and displacement of the other two missiles when inferno firing
  // (assuming symmetry around central missile)
  pub missile_inferno_angle: Rotation,
  pub missile_inferno_offset_x: Distance,
  pub missile_inferno_offset_y: Distance,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, DeserializeOver)]
pub struct MissileInfo {
  pub max_speed: Speed,
  pub accel: AccelScalar,
  pub base_speed: Speed,
  pub speed_factor: f32,
  pub damage: Health,
  pub distance: Distance,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, DeserializeOver)]
pub struct MobInfo {
  #[serde(with = "option_duration")]
  pub lifetime: Option<Duration>,
  #[deserialize_over]
  pub missile: Option<MissileInfo>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, DeserializeOver)]
pub struct UpgradeInfo {
  pub cost: [u16; 6],
  pub factor: [f32; 6],
}

#[derive(Clone, Debug, Serialize, Deserialize, DeserializeOver)]
pub struct PlaneInfos {
  #[deserialize_over]
  pub predator: PlaneInfo,
  #[deserialize_over]
  pub tornado: PlaneInfo,
  #[deserialize_over]
  pub prowler: PlaneInfo,
  #[deserialize_over]
  pub mohawk: PlaneInfo,
  #[deserialize_over]
  pub goliath: PlaneInfo,
}
#[derive(Clone, Debug, Serialize, Deserialize, DeserializeOver)]
pub struct MobInfos {
  #[deserialize_over]
  pub predator: MobInfo,
  #[deserialize_over]
  pub tornado: MobInfo,
  #[deserialize_over]
  pub prowler: MobInfo,
  #[deserialize_over]
  pub mohawk: MobInfo,
  #[deserialize_over]
  pub goliath: MobInfo,
  #[deserialize_over]
  pub tornado_triple: MobInfo,
  #[deserialize_over]
  pub upgrade: MobInfo,
  #[deserialize_over]
  pub inferno: MobInfo,
  #[deserialize_over]
  pub shield: MobInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeserializeOver)]
pub struct UpgradeInfos {
  #[deserialize_over]
  pub speed: UpgradeInfo,
  #[deserialize_over]
  pub missile: UpgradeInfo,
  #[deserialize_over]
  pub energy: UpgradeInfo,
  #[deserialize_over]
  pub defense: UpgradeInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeserializeOver)]
pub struct Config {
  #[deserialize_over]
  #[deprecated]
  pub planes: PlaneInfos,
  #[deserialize_over]
  #[deprecated]
  pub mobs: MobInfos,
  #[deserialize_over]
  #[deprecated]
  pub upgrades: UpgradeInfos,

  #[serde(with = "duration")]
  pub spawn_shield_duration: Duration,
  #[serde(with = "duration")]
  pub shield_duration: Duration,
  #[serde(with = "duration")]
  pub inferno_duration: Duration,

  #[deprecated]
  /// The radius in which the player can observe events happening.
  pub view_radius: Distance,

  /// The delay between a player dying and them being allowed to respawn.
  #[serde(with = "duration")]
  #[deprecated]
  pub respawn_delay: Duration,
}

impl MobInfos {
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MobInfo> {
    let arr = vec![
      &mut self.predator,
      &mut self.tornado,
      &mut self.prowler,
      &mut self.mohawk,
      &mut self.goliath,
      &mut self.tornado_triple,
      &mut self.upgrade,
      &mut self.inferno,
      &mut self.shield,
    ];

    arr.into_iter()
  }
}

impl Index<Plane> for PlaneInfos {
  type Output = PlaneInfo;

  fn index(&self, idx: Plane) -> &PlaneInfo {
    match idx {
      Plane::Predator => &self.predator,
      Plane::Tornado => &self.tornado,
      Plane::Prowler => &self.prowler,
      Plane::Mohawk => &self.mohawk,
      Plane::Goliath => &self.goliath,
      _ => panic!("got unexpected plane type {:?}", idx),
    }
  }
}

impl Index<Mob> for MobInfos {
  type Output = MobInfo;

  fn index(&self, idx: Mob) -> &MobInfo {
    match idx {
      Mob::PredatorMissile => &self.predator,
      Mob::TornadoSingleMissile => &self.tornado,
      Mob::ProwlerMissile => &self.prowler,
      Mob::MohawkMissile => &self.mohawk,
      Mob::GoliathMissile => &self.goliath,
      Mob::TornadoTripleMissile => &self.tornado_triple,
      Mob::Upgrade => &self.upgrade,
      Mob::Inferno => &self.inferno,
      Mob::Shield => &self.shield,
      _ => panic!("got unexpected mob type {:?}", idx),
    }
  }
}

impl Default for PlaneInfos {
  fn default() -> Self {
    use self::plane_defaults::*;

    Self {
      predator: predator(),
      goliath: goliath(),
      mohawk: mohawk(),
      tornado: tornado(),
      prowler: prowler(),
    }
  }
}

impl Default for MobInfos {
  fn default() -> Self {
    use self::mob_defaults::*;

    Self {
      predator: predator_missile(),
      tornado: tornado_single_missile(),
      prowler: prowler_missile(),
      mohawk: mohawk_missile(),
      goliath: goliath_missile(),
      tornado_triple: tornado_triple_missile(),
      upgrade: upgrade(),
      inferno: inferno(),
      shield: shield(),
    }
  }
}

impl Default for UpgradeInfos {
  fn default() -> Self {
    const N0: u16 = 0;
    const N1: u16 = 1;

    Self {
      speed: UpgradeInfo {
        cost: [N0, N1, N1, N1, N1, N1],
        factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
      },
      defense: UpgradeInfo {
        cost: [N0, N1, N1, N1, N1, N1],
        factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
      },
      energy: UpgradeInfo {
        cost: [N0, N1, N1, N1, N1, N1],
        factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
      },
      missile: UpgradeInfo {
        cost: [N0, N1, N1, N1, N1, N1],
        factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
      },
    }
  }
}

#[allow(deprecated)]
impl Default for Config {
  fn default() -> Self {
    Self {
      planes: Default::default(),
      mobs: Default::default(),
      upgrades: Default::default(),
      spawn_shield_duration: Duration::from_secs(2),
      shield_duration: Duration::from_secs(10),
      inferno_duration: Duration::from_secs(10),
      view_radius: 2250.0,
      respawn_delay: Duration::from_secs(2),
    }
  }
}

mod plane_defaults {
  use super::*;

  pub(super) fn predator() -> PlaneInfo {
    PlaneInfo {
      turn_factor: 0.065,

      accel_factor: 0.225,
      brake_factor: 0.025,
      boost_factor: 1.5,

      max_speed: 5.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,

      health_regen: 0.001,
      energy_regen: 0.008,
      fire_delay: Duration::from_millis(550),

      damage_factor: 2.0,

      fire_energy: 0.6,

      missile_type: MobType::PredatorMissile,
      missile_offset: vector![35.0, 0.0],

      missile_inferno_angle: 0.05,
      missile_inferno_offset_x: 18.0,
      missile_inferno_offset_y: 1.25,
    }
  }
  pub(super) fn goliath() -> PlaneInfo {
    PlaneInfo {
      turn_factor: 0.04,

      accel_factor: 0.15,
      brake_factor: 0.015,
      boost_factor: 1.0,

      max_speed: 3.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,

      health_regen: 0.0005,
      energy_regen: 0.005,
      fire_delay: Duration::from_millis(300),

      damage_factor: 1.0,

      fire_energy: 0.9,

      missile_type: MobType::GoliathMissile,
      missile_offset: vector![35.0, 0.0],

      missile_inferno_angle: 0.04,
      missile_inferno_offset_x: 30.0,
      missile_inferno_offset_y: 2.1,
    }
  }
  pub(super) fn mohawk() -> PlaneInfo {
    PlaneInfo {
      turn_factor: 0.07,

      accel_factor: 0.275,
      brake_factor: 0.025,
      boost_factor: 1.0,

      max_speed: 6.0,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,

      health_regen: 0.001,
      energy_regen: 0.01,
      fire_delay: Duration::from_millis(300),

      damage_factor: 2.6375,

      fire_energy: 0.3,

      missile_type: MobType::MohawkMissile,
      // This will have to be a special case
      missile_offset: vector![10.0, 15.0],

      missile_inferno_angle: 0.1,
      missile_inferno_offset_x: 0.0,
      missile_inferno_offset_y: 0.0,
    }
  }
  pub(super) fn tornado() -> PlaneInfo {
    PlaneInfo {
      turn_factor: 0.055,

      accel_factor: 0.2,
      brake_factor: 0.025,
      boost_factor: 1.0,

      max_speed: 4.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,

      health_regen: 0.001,
      energy_regen: 0.006,
      fire_delay: Duration::from_millis(500),

      damage_factor: 5.0 / 3.0,

      fire_energy: 0.5,

      missile_type: MobType::TornadoSingleMissile,
      missile_offset: vector![40.0, 0.0],

      missile_inferno_angle: 0.05,
      missile_inferno_offset_x: 15.1,
      missile_inferno_offset_y: 10.0,
    }
  }
  pub(super) fn prowler() -> PlaneInfo {
    PlaneInfo {
      turn_factor: 0.055,

      accel_factor: 0.2,
      brake_factor: 0.025,
      boost_factor: 1.0,

      max_speed: 4.5,
      min_speed: 0.001,
      flag_speed: 5.0,
      inferno_factor: 0.75,

      health_regen: 0.001,
      energy_regen: 0.006,
      fire_delay: Duration::from_millis(300),

      damage_factor: 5.0 / 3.0,

      fire_energy: 0.75,

      missile_type: MobType::ProwlerMissile,
      missile_offset: vector![35.0, 0.0],

      missile_inferno_angle: 0.05,
      missile_inferno_offset_x: 18.0,
      missile_inferno_offset_y: 2.25,
    }
  }
}

mod mob_defaults {
  use super::*;

  // Notes:
  //   - Damage is normalized to the amount of damage that would be done to a
  //     goliath.
  //   - This will then be multiplied by a factor specific to each plane type

  pub(super) fn predator_missile() -> MobInfo {
    MobInfo {
      lifetime: None,
      missile: Some(MissileInfo {
        max_speed: 9.0,
        accel: 0.105,
        base_speed: 4.05,
        speed_factor: 0.3,
        damage: 0.4,
        distance: 1104.0,
      }),
    }
  }
  pub(super) fn goliath_missile() -> MobInfo {
    MobInfo {
      lifetime: None,
      missile: Some(MissileInfo {
        max_speed: 6.0,
        accel: 0.0375,
        base_speed: 2.1,
        speed_factor: 0.3,
        damage: 1.2,
        distance: 1076.0,
      }),
    }
  }
  pub(super) fn mohawk_missile() -> MobInfo {
    MobInfo {
      lifetime: None,
      missile: Some(MissileInfo {
        max_speed: 9.0,
        accel: 0.14,
        base_speed: 5.7,
        speed_factor: 0.3,
        damage: 0.2,
        distance: 1161.0,
      }),
    }
  }
  pub(super) fn tornado_single_missile() -> MobInfo {
    MobInfo {
      lifetime: None,
      missile: Some(MissileInfo {
        max_speed: 7.0,
        accel: 0.0875,
        base_speed: 3.5,
        speed_factor: 0.3,
        damage: 0.4,
        distance: 997.0,
      }),
    }
  }
  pub(super) fn tornado_triple_missile() -> MobInfo {
    MobInfo {
      lifetime: None,
      missile: Some(MissileInfo {
        max_speed: 7.0,
        accel: 0.0875,
        base_speed: 3.5,
        speed_factor: 0.3,
        damage: 0.3,
        distance: 581.0,
      }),
    }
  }
  pub(super) fn prowler_missile() -> MobInfo {
    MobInfo {
      lifetime: None,
      missile: Some(MissileInfo {
        max_speed: 7.0,
        accel: 0.07,
        base_speed: 2.8,
        speed_factor: 0.3,
        damage: 0.45,
        distance: 819.0,
      }),
    }
  }

  pub(super) fn inferno() -> MobInfo {
    MobInfo {
      lifetime: Some(Duration::from_secs(60)),
      missile: None,
    }
  }
  pub(super) fn shield() -> MobInfo {
    MobInfo {
      lifetime: Some(Duration::from_secs(60)),
      missile: None,
    }
  }
  pub(super) fn upgrade() -> MobInfo {
    MobInfo {
      lifetime: Some(Duration::from_secs(60)),
      missile: None,
    }
  }
}
