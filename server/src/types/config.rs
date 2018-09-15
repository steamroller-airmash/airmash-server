use airmash_protocol::{MobType, PlaneType};
use fnv::FnvHashMap;
use std::ops::Index;
use std::time::Duration;

use types::*;

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
	pub cost: [UpgradeCount; 6],
	pub factor: [f32; 6],
}

#[derive(Clone, Debug)]
pub struct PlaneInfos(pub FnvHashMap<Plane, PlaneInfo>);
#[derive(Clone, Debug)]
pub struct MobInfos(pub FnvHashMap<MobType, MobInfo>);

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
	pub spawn_shield_duration: Duration,
	pub shield_duration: Duration,
	pub inferno_duration: Duration,
}

impl Index<Plane> for PlaneInfos {
	type Output = PlaneInfo;

	fn index(&self, idx: Plane) -> &PlaneInfo {
		&self.0[&idx]
	}
}

impl Index<Mob> for MobInfos {
	type Output = MobInfo;

	fn index(&self, idx: Mob) -> &MobInfo {
		&self.0[&idx]
	}
}

impl Default for PlaneInfos {
	fn default() -> Self {
		let mut map = FnvHashMap::default();

		map.insert(
			PlaneType::Predator,
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

				missile_type: MobType::PredatorMissile,
				missile_offset: Distance::new(35.0),
			},
		);

		map.insert(
			PlaneType::Goliath,
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

				missile_type: MobType::GoliathMissile,
				missile_offset: Distance::new(35.0),
			},
		);

		map.insert(
			PlaneType::Mohawk,
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

				missile_type: MobType::MohawkMissile,
				// This will have to be a special case
				missile_offset: Distance::new(10.0),
			},
		);

		map.insert(
			PlaneType::Tornado,
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

				missile_type: MobType::TornadoSingleMissile,
				missile_offset: Distance::new(40.0),
			},
		);

		map.insert(
			PlaneType::Prowler,
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

				missile_type: MobType::ProwlerMissile,
				missile_offset: Distance::new(35.0),
			},
		);

		PlaneInfos(map)
	}
}

impl Default for MobInfos {
	fn default() -> Self {
		let mut map = FnvHashMap::default();

		/*
		Notes:
			- Damage is normalized to the amount of
			  damage that would be done to a goliath.
			- This will then be multiplied by a factor
			  specific to each plane type
		*/

		map.insert(
			MobType::PredatorMissile,
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
			},
		);

		map.insert(
			MobType::GoliathMissile,
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
			},
		);

		map.insert(
			MobType::MohawkMissile,
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
			},
		);

		map.insert(
			MobType::TornadoSingleMissile,
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
			},
		);

		map.insert(
			MobType::TornadoTripleMissile,
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
			},
		);

		map.insert(
			MobType::ProwlerMissile,
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
			},
		);

		// TODO: Determine actual powerup lifetime
		map.insert(
			MobType::Inferno,
			MobInfo {
				lifetime: Some(Duration::from_secs(60)),
				missile: None,
			},
		);

		map.insert(
			MobType::Shield,
			MobInfo {
				lifetime: Some(Duration::from_secs(60)),
				missile: None,
			},
		);

		MobInfos(map)
	}
}

impl Default for UpgradeInfos {
	fn default() -> Self {
		const N0: UpgradeCount = UpgradeCount(0);
		const N1: UpgradeCount = UpgradeCount(1);

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

impl Default for Config {
	fn default() -> Self {
		Self {
			planes: Default::default(),
			mobs: Default::default(),
			upgrades: Default::default(),
			admin_enabled: true,
			spawn_shield_duration: Duration::from_secs(2),
			shield_duration: Duration::from_secs(10),
			inferno_duration: Duration::from_secs(10),
		}
	}
}
