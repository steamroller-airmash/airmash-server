use crate::{Distance, Plane, Position};

use std::ops::Index;

#[derive(Copy, Clone, Debug, Default)]
pub struct HitCircle {
    pub radius: Distance,
    pub offset: Position,
}

macro_rules! hit_circle {
    ($x:expr, $y:expr, $r:expr) => {
        HitCircle {
            offset: Position {
                x: Distance::new($x as f32),
                y: Distance::new($y as f32),
            },
            radius: Distance::new($r as f32),
        }
    };
}

const PRED_HC: &'static [HitCircle] = &[
    hit_circle!(0, 5, 23),
    hit_circle!(0, -15, 15),
    hit_circle!(0, -25, 12),
];

const GOLI_HC: &'static [HitCircle] = &[
    hit_circle!(0, 0, 35),
    hit_circle!(50, 14, 16),
    hit_circle!(74, 26, 14),
    hit_circle!(30, 8, 23),
    hit_circle!(63, 22, 15),
    hit_circle!(-50, 14, 16),
    hit_circle!(-74, 26, 14),
    hit_circle!(-30, 8, 23),
    hit_circle!(-63, 22, 15),
];

const MOHAWK_HC: &'static [HitCircle] = &[
    hit_circle!(0, -12, 15),
    hit_circle!(0, 0, 17),
    hit_circle!(0, 13, 15),
    hit_circle!(0, 26, 15),
];

const TORNADO_HC: &'static [HitCircle] = &[
    hit_circle!(0, 8, 18),
    hit_circle!(14, 12, 13),
    hit_circle!(-14, 12, 13),
    hit_circle!(0, -12, 16),
    hit_circle!(0, -26, 14),
    hit_circle!(0, -35, 12),
];

const PROWLER_HC: &'static [HitCircle] = &[
    hit_circle!(0, 11, 25),
    hit_circle!(0, -8, 18),
    hit_circle!(19, 20, 10),
    hit_circle!(-19, 20, 10),
    hit_circle!(0, -20, 14),
];

pub const PLANE_HIT_CIRCLES: PlaneMap = PlaneMap(());

pub struct PlaneMap(());

impl Index<Plane> for PlaneMap {
    type Output = [HitCircle];

    fn index(&self, idx: Plane) -> &Self::Output {
        match idx {
            Plane::Predator => PRED_HC,
            Plane::Goliath => GOLI_HC,
            Plane::Mohawk => MOHAWK_HC,
            Plane::Tornado => TORNADO_HC,
            Plane::Prowler => PROWLER_HC,
        }
    }
}
