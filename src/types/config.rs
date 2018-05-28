use airmash_protocol::PlaneType;
use fnv::FnvHashMap;
use std::ops::Index;
use std::vec::Vec;

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
    pub max_speed: SpeedScalar,
    pub min_speed: SpeedScalar,
    pub flag_speed: SpeedScalar,
    pub inferno_factor: f32,

    // Regen
    pub health_regen: HealthRegen,
    pub energy_regen: EnergyRegen,

    // Collisions
    pub hit_circles: Vec<HitCircle>,
}

#[derive(Clone)]
pub struct PlaneInfos(pub FnvHashMap<Plane, PlaneInfo>);

#[derive(Clone, Default)]
pub struct Config {
    pub planes: PlaneInfos,
}

impl Index<Plane> for PlaneInfos {
    type Output = PlaneInfo;

    fn index(&self, idx: Plane) -> &PlaneInfo {
        &self.0[&idx]
    }
}

impl Default for PlaneInfos {
    fn default() -> Self {
        let mut map = FnvHashMap::default();

        map.insert(
            Plane(PlaneType::Predator),
            PlaneInfo {
                turn_factor: RotationRate::new(0.065),

                accel_factor: AccelScalar::new(0.225),
                brake_factor: AccelScalar::new(0.025),
                boost_factor: 1.5,

                max_speed: SpeedScalar::new(5.5),
                min_speed: SpeedScalar::new(0.1),
                flag_speed: SpeedScalar::new(5.0),
                inferno_factor: 0.75,

                // TODO: Set these
                health_regen: HealthRegen::new(0.0),
                energy_regen: EnergyRegen::new(0.0),

                // Also TODO
                hit_circles: vec![],
            },
        );

        // TODO: Other Planes

        PlaneInfos(map)
    }
}
