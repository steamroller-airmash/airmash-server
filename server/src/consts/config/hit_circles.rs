use hashbrown::HashMap;

use crate::{Distance, Plane, Position};

use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, Default)]
pub struct HitCircle {
  pub radius: Distance,
  pub offset: Position,
}

impl HitCircle {
  pub fn new(x: i16, y: i16, r: i16) -> Self {
    Self {
      offset: Position::new(Distance::new(x as f32), Distance::new(y as f32)),
      radius: Distance::new(r as f32),
    }
  }
}

macro_rules! hit_circle {
  ($x:expr, $y:expr, $r:expr) => {
    HitCircle {
      offset: Position {
        x: Distance {
          value_unsafe: $x as f32,
          _marker: PhantomData,
        },
        y: Distance {
          value_unsafe: $y as f32,
          _marker: PhantomData,
        },
      },
      radius: Distance {
        value_unsafe: $r as f32,
        _marker: PhantomData,
      },
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

lazy_static! {
  pub static ref PLANE_HIT_CIRCLES: HashMap<Plane, &'static [HitCircle]> = {
    use Plane::*;

    let mut map = HashMap::default();

    map.insert(Predator, PRED_HC);
    map.insert(Goliath, GOLI_HC);
    map.insert(Mohawk, MOHAWK_HC);
    map.insert(Tornado, TORNADO_HC);
    map.insert(Prowler, PROWLER_HC);

    map
  };
}
