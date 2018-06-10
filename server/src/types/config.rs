use airmash_protocol::{PlaneType, MobType};
use fnv::FnvHashMap;
use std::ops::Index;
use std::vec::Vec;
use std::time::Duration;

use types::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct HitCircle {
	pub radius: Distance,
	pub offset: Position,
}

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

	// Energy requirement
	pub fire_energy: Energy,
	pub special_energy: Energy,

	// Type of missile that the plane fires
	pub missile_type: Mob,

	// Collisions
	pub hit_circles: Vec<HitCircle>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MissileInfo {
	pub max_speed: Speed,
	pub accel: AccelScalar,
	pub base_speed: Speed,
	pub speed_factor: f32
}

#[derive(Clone, Debug, Default)]
pub struct MobInfo {
	pub lifetime: Duration,
	pub hit_circles: Vec<HitCircle>,
	pub missile: Option<MissileInfo>
}

#[derive(Clone)]
pub struct PlaneInfos(pub FnvHashMap<Plane, PlaneInfo>);
#[derive(Clone)]
pub struct MobInfos(pub FnvHashMap<MobType, MobInfo>);

#[derive(Clone, Default)]
pub struct Config {
	pub planes: PlaneInfos,
	pub mobs: MobInfos
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

fn hit_circle(x: i16, y: i16, r: i16) -> HitCircle {
	HitCircle {
		offset: Position::new(
			Distance::new(x as f32),
			Distance::new(y as f32)
		),
		radius: Distance::new(r as f32)
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

				// TODO: Set these
				health_regen: HealthRegen::new(0.0),
				energy_regen: EnergyRegen::new(0.0),

				// TODO
				fire_energy: Energy::new(0.0),
				special_energy: Energy::new(0.0),

				missile_type: MobType::PredatorMissile,

				hit_circles: vec![
					hit_circle(0, 5, 23),
					hit_circle(0, -15, 15),
					hit_circle(0, -25, 12),
				],
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

				// TODO: Set these
				health_regen: HealthRegen::new(0.0),
				energy_regen: EnergyRegen::new(0.0),

				// TODO
				fire_energy: Energy::new(0.0),
				special_energy: Energy::new(0.0),

				missile_type: MobType::GoliathMissile,

				hit_circles: vec![
					hit_circle( 0, 0, 35),
					hit_circle(50, 14, 16),
					hit_circle(74, 26, 14),
					hit_circle(30, 8, 23),
					hit_circle(63, 22, 15),
					hit_circle(-50, 14, 16),
					hit_circle(-74, 26, 14),
					hit_circle(-30, 8, 23),
					hit_circle(-63, 22, 15)
				]
			}
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
				
				// TODO: Set these
				health_regen: HealthRegen::new(0.0),
				energy_regen: EnergyRegen::new(0.0),

				// TODO
				fire_energy: Energy::new(0.0),
				special_energy: Energy::new(0.0),

				missile_type: MobType::MohawkMissile,

				hit_circles: vec![
					hit_circle(0, -12, 15),
					hit_circle(0, 0, 17),
					hit_circle(0, 13, 15),
					hit_circle(0, 26, 15)
				]
			}
		);

		map.insert(
			PlaneType::Tornado,
			PlaneInfo {
				turn_factor: RotationRate::new(0.055),

				accel_factor: AccelScalar::new(0.2),
				brake_factor: AccelScalar::new(0.025),
				boost_factor: 1.0,

				max_speed: Speed::new(6.0),
				min_speed: Speed::new(0.001),
				flag_speed: Speed::new(5.0),
				inferno_factor: 0.75,
				
				// TODO: Set these
				health_regen: HealthRegen::new(0.0),
				energy_regen: EnergyRegen::new(0.0),

				// TODO
				fire_energy: Energy::new(0.0),
				special_energy: Energy::new(0.0),

				missile_type: MobType::TornadoSingleMissile,

				hit_circles: vec![
					hit_circle(0, 8, 18),
					hit_circle(14, 12, 13),
					hit_circle(-14, 12, 13),
					hit_circle(0, -12, 16),
					hit_circle(0, -26, 14),
					hit_circle(0, -35, 12)
				]
			}
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

				// TODO: Set these
				health_regen: HealthRegen::new(0.0),
				energy_regen: EnergyRegen::new(0.0),

				// TODO
				fire_energy: Energy::new(0.0),
				special_energy: Energy::new(0.0),

				missile_type: MobType::ProwlerMissile,

				hit_circles: vec![
					hit_circle(0, 11, 25),
					hit_circle(0, -8, 18),
					hit_circle(19, 20, 10),
					hit_circle(-19, 20, 10),
					hit_circle(0, -20, 14)
				]
			}
		);

		PlaneInfos(map)
	}
}

impl Default for MobInfos {
	fn default() -> Self {
		let mut map = FnvHashMap::default();

		map.insert(
			MobType::PredatorMissile,
			MobInfo {
				// TODO: Figure out missile lifetime
				lifetime: Duration::from_secs(10),
				missile: Some(MissileInfo {
					max_speed: Speed::new(9.0),
					accel: AccelScalar::new(0.105),
					base_speed: Speed::new(4.05),
					speed_factor: 0.3
				}),
				hit_circles: vec![]
			}
		);

		map.insert(
			MobType::GoliathMissile,
			MobInfo {
				lifetime: Duration::from_secs(10),
				missile: Some(MissileInfo {
					max_speed: Speed::new(6.0),
					accel: AccelScalar::new(0.0375),
					base_speed: Speed::new(2.1),
					speed_factor: 0.3
				}),
				hit_circles: vec![]
			}
		);

		map.insert(
			MobType::MohawkMissile,
			MobInfo {
				lifetime: Duration::from_secs(10),
				missile: Some(MissileInfo {
					max_speed: Speed::new(9.0),
					accel: AccelScalar::new(0.14),
					base_speed: Speed::new(5.7),
					speed_factor: 0.3
				}),
				hit_circles: vec![]
			}
		);

		map.insert(
			MobType::TornadoSingleMissile,
			MobInfo {
				lifetime: Duration::from_secs(10),
				missile: Some(MissileInfo {
					max_speed: Speed::new(7.0),
					accel: AccelScalar::new(0.0875),
					base_speed: Speed::new(3.5),
					speed_factor: 0.3
				}),
				hit_circles: vec![]
			}
		);

		map.insert(
			MobType::TornadoTripleMissile,
			MobInfo {
				lifetime: Duration::from_secs(10),
				missile: Some(MissileInfo {
					max_speed: Speed::new(7.0),
					accel: AccelScalar::new(0.0875),
					base_speed: Speed::new(3.5),
					speed_factor: 0.3
				}),
				hit_circles: vec![]
			}
		);

		map.insert(
			MobType::ProwlerMissile,
			MobInfo {
				lifetime: Duration::from_secs(10),
				missile: Some(MissileInfo {
					max_speed: Speed::new(7.0),
					accel: AccelScalar::new(0.07),
					base_speed: Speed::new(2.8),
					speed_factor: 0.3
				}),
				hit_circles: vec![]
			}
		);

		// TODO: Powerups

		MobInfos(map)
	}
}
