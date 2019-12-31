use std::ops::Index;
use std::time::Duration;

use crate::*;

#[derive(Debug, Clone)]
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
    pub fire_delay: Duration,

    // Type of missile that the plane fires
    pub missile_type: Mob,
    // Offset of missile (in the Y dir) when fired
    pub missile_offset: Distance,

    // Angle and displacement of the other two missiles when inferno firing
    // (assuming symmetry around central missile)
    pub missile_inferno_angle: Rotation,
    pub missile_inferno_offset_x: Distance,
    pub missile_inferno_offset_y: Distance,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MissileInfo {
    pub max_speed: Speed,
    pub accel: AccelScalar,
    pub base_speed: Speed,
    pub speed_factor: f32,
    pub damage: Health,
    pub distance: Distance,
}

#[derive(Clone, Debug, Default)]
pub struct MobInfo {
    pub lifetime: Option<Duration>,
    pub missile: Option<MissileInfo>,
}

#[derive(Clone, Debug, Default)]
pub struct UpgradeInfo {
    pub cost: [u16; 6],
    pub factor: [f32; 6],
}

#[derive(Clone, Debug)]
pub struct PlaneInfos {
    pub predator: PlaneInfo,
    pub tornado: PlaneInfo,
    pub prowler: PlaneInfo,
    pub mohawk: PlaneInfo,
    pub goliath: PlaneInfo,
}
#[derive(Clone, Debug)]
pub struct MobInfos {
    pub predator: MobInfo,
    pub tornado: MobInfo,
    pub prowler: MobInfo,
    pub mohawk: MobInfo,
    pub goliath: MobInfo,
    pub tornado_triple: MobInfo,
    pub upgrade: MobInfo,
    pub inferno: MobInfo,
    pub shield: MobInfo,
}

#[derive(Clone, Debug)]
pub struct UpgradeInfos {
    pub speed: UpgradeInfo,
    pub missile: UpgradeInfo,
    pub energy: UpgradeInfo,
    pub defense: UpgradeInfo,
}

#[derive(Clone)]
pub struct Config {
    pub planes: PlaneInfos,
    pub mobs: MobInfos,
    pub upgrades: UpgradeInfos,

    pub admin_enabled: bool,
    pub allow_spectate_while_moving: bool,
    pub spawn_shield_duration: Duration,
    pub shield_duration: Duration,
    pub inferno_duration: Duration,
    pub view_radius: Distance,
    pub afk_timeout: Duration,
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
        Self {
            speed: UpgradeInfo {
                cost: [0, 1, 1, 1, 1, 1],
                factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
            },
            defense: UpgradeInfo {
                cost: [0, 1, 1, 1, 1, 1],
                factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
            },
            energy: UpgradeInfo {
                cost: [0, 1, 1, 1, 1, 1],
                factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
            },
            missile: UpgradeInfo {
                cost: [0, 1, 1, 1, 1, 1],
                factor: [1.0, 1.05, 1.1, 1.15, 1.2, 1.25],
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            planes: Default::default(),
            mobs: Default::default(),
            upgrades: Default::default(),
            // This is a nasty bodge, but there seems to be no more appropriate
            // default feature defined. Ensure admin commands are disabled by
            // default in release mode, unless explicitly requested.
            admin_enabled: cfg!(debug_assertions),
            allow_spectate_while_moving: true,
            spawn_shield_duration: Duration::from_secs(2),
            shield_duration: Duration::from_secs(10),
            inferno_duration: Duration::from_secs(10),
            view_radius: Distance::new(2250.0),
            afk_timeout: Duration::from_secs(60 * 20),
        }
    }
}

mod plane_defaults {
    use super::*;

    pub(super) fn predator() -> PlaneInfo {
        PlaneInfo {
            turn_factor: RotationRate::new(0.065),

            accel_factor: AccelScalar::new(0.225),
            brake_factor: AccelScalar::new(0.025),
            boost_factor: 1.5,

            max_speed: Speed::new(5.5),
            min_speed: Speed::new(0.001),
            flag_speed: Speed::new(5.0),
            inferno_factor: 0.75,

            health_regen: HealthRegen::new(0.001),
            energy_regen: EnergyRegen::new(0.008),
            fire_delay: Duration::from_millis(550),

            damage_factor: 2.0,

            fire_energy: Energy::new(0.6),

            missile_type: Mob::PredatorMissile,
            missile_offset: Distance::new(35.0),

            missile_inferno_angle: Rotation::new(0.05),
            missile_inferno_offset_x: Distance::new(18.0),
            missile_inferno_offset_y: Distance::new(1.25),
        }
    }
    pub(super) fn goliath() -> PlaneInfo {
        PlaneInfo {
            turn_factor: RotationRate::new(0.04),

            accel_factor: AccelScalar::new(0.15),
            brake_factor: AccelScalar::new(0.015),
            boost_factor: 1.0,

            max_speed: Speed::new(3.5),
            min_speed: Speed::new(0.001),
            flag_speed: Speed::new(5.0),
            inferno_factor: 0.75,

            health_regen: HealthRegen::new(0.0005),
            energy_regen: EnergyRegen::new(0.005),
            fire_delay: Duration::from_millis(300),

            damage_factor: 1.0,

            fire_energy: Energy::new(0.9),

            missile_type: Mob::GoliathMissile,
            missile_offset: Distance::new(35.0),

            missile_inferno_angle: Rotation::new(0.04),
            missile_inferno_offset_x: Distance::new(30.0),
            missile_inferno_offset_y: Distance::new(2.1),
        }
    }
    pub(super) fn mohawk() -> PlaneInfo {
        PlaneInfo {
            turn_factor: RotationRate::new(0.07),

            accel_factor: AccelScalar::new(0.275),
            brake_factor: AccelScalar::new(0.025),
            boost_factor: 1.0,

            max_speed: Speed::new(6.0),
            min_speed: Speed::new(0.001),
            flag_speed: Speed::new(5.0),
            inferno_factor: 0.75,

            health_regen: HealthRegen::new(0.001),
            energy_regen: EnergyRegen::new(0.01),
            fire_delay: Duration::from_millis(300),

            damage_factor: 2.6375,

            fire_energy: Energy::new(0.3),

            missile_type: Mob::MohawkMissile,
            // This will have to be a special case
            missile_offset: Distance::new(10.0),

            missile_inferno_angle: Rotation::new(0.1),
            missile_inferno_offset_x: Distance::new(0.0),
            missile_inferno_offset_y: Distance::new(0.0),
        }
    }
    pub(super) fn tornado() -> PlaneInfo {
        PlaneInfo {
            turn_factor: RotationRate::new(0.055),

            accel_factor: AccelScalar::new(0.2),
            brake_factor: AccelScalar::new(0.025),
            boost_factor: 1.0,

            max_speed: Speed::new(4.5),
            min_speed: Speed::new(0.001),
            flag_speed: Speed::new(5.0),
            inferno_factor: 0.75,

            health_regen: HealthRegen::new(0.001),
            energy_regen: EnergyRegen::new(0.006),
            fire_delay: Duration::from_millis(500),

            damage_factor: 5.0 / 3.0,

            fire_energy: Energy::new(0.5),

            missile_type: Mob::TornadoSingleMissile,
            missile_offset: Distance::new(40.0),

            missile_inferno_angle: Rotation::new(0.05),
            missile_inferno_offset_x: Distance::new(15.1),
            missile_inferno_offset_y: Distance::new(10.0),
        }
    }
    pub(super) fn prowler() -> PlaneInfo {
        PlaneInfo {
            turn_factor: RotationRate::new(0.055),

            accel_factor: AccelScalar::new(0.2),
            brake_factor: AccelScalar::new(0.025),
            boost_factor: 1.0,

            max_speed: Speed::new(4.5),
            min_speed: Speed::new(0.001),
            flag_speed: Speed::new(5.0),
            inferno_factor: 0.75,

            health_regen: HealthRegen::new(0.001),
            energy_regen: EnergyRegen::new(0.006),
            fire_delay: Duration::from_millis(300),

            damage_factor: 5.0 / 3.0,

            fire_energy: Energy::new(0.75),

            missile_type: Mob::ProwlerMissile,
            missile_offset: Distance::new(35.0),

            missile_inferno_angle: Rotation::new(0.05),
            missile_inferno_offset_x: Distance::new(18.0),
            missile_inferno_offset_y: Distance::new(2.25),
        }
    }
}

mod mob_defaults {
    use super::*;

    // Notes:
    //   - Damage is normalized to the amount of
    //     damage that would be done to a goliath.
    //   - This will then be multiplied by a factor
    //     specific to each plane type

    pub(super) fn predator_missile() -> MobInfo {
        MobInfo {
            lifetime: None,
            missile: Some(MissileInfo {
                max_speed: Speed::new(9.0),
                accel: AccelScalar::new(0.105),
                base_speed: Speed::new(4.05),
                speed_factor: 0.3,
                damage: Health::new(0.4),
                distance: Distance::new(1104.0),
            }),
        }
    }
    pub(super) fn goliath_missile() -> MobInfo {
        MobInfo {
            lifetime: None,
            missile: Some(MissileInfo {
                max_speed: Speed::new(6.0),
                accel: AccelScalar::new(0.0375),
                base_speed: Speed::new(2.1),
                speed_factor: 0.3,
                damage: Health::new(1.2),
                distance: Distance::new(1076.0),
            }),
        }
    }
    pub(super) fn mohawk_missile() -> MobInfo {
        MobInfo {
            lifetime: None,
            missile: Some(MissileInfo {
                max_speed: Speed::new(9.0),
                accel: AccelScalar::new(0.14),
                base_speed: Speed::new(5.7),
                speed_factor: 0.3,
                damage: Health::new(0.2),
                distance: Distance::new(1161.0),
            }),
        }
    }
    pub(super) fn tornado_single_missile() -> MobInfo {
        MobInfo {
            lifetime: None,
            missile: Some(MissileInfo {
                max_speed: Speed::new(7.0),
                accel: AccelScalar::new(0.0875),
                base_speed: Speed::new(3.5),
                speed_factor: 0.3,
                damage: Health::new(0.4),
                distance: Distance::new(997.0),
            }),
        }
    }
    pub(super) fn tornado_triple_missile() -> MobInfo {
        MobInfo {
            lifetime: None,
            missile: Some(MissileInfo {
                max_speed: Speed::new(7.0),
                accel: AccelScalar::new(0.0875),
                base_speed: Speed::new(3.5),
                speed_factor: 0.3,
                damage: Health::new(0.3),
                distance: Distance::new(581.0),
            }),
        }
    }
    pub(super) fn prowler_missile() -> MobInfo {
        MobInfo {
            lifetime: None,
            missile: Some(MissileInfo {
                max_speed: Speed::new(7.0),
                accel: AccelScalar::new(0.07),
                base_speed: Speed::new(2.8),
                speed_factor: 0.3,
                damage: Health::new(0.45),
                distance: Distance::new(819.0),
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
