use crate::protocol::PlaneType;
use crate::Vector2;

type HitCircle = (Vector2<f32>, f32);

macro_rules! hit_circle {
  ($x:expr, $y:expr, $r:expr) => {
    (Vector2::new($x as f32, $y as f32), $r as f32)
  };
}

const PRED_HC: &[HitCircle] = &[
  hit_circle!(0, 5, 23),
  hit_circle!(0, -15, 15),
  hit_circle!(0, -25, 12),
];

const GOLI_HC: &[HitCircle] = &[
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

const MOHAWK_HC: &[HitCircle] = &[
  hit_circle!(0, -12, 15),
  hit_circle!(0, 0, 17),
  hit_circle!(0, 13, 15),
  hit_circle!(0, 26, 15),
];

const TORNADO_HC: &[HitCircle] = &[
  hit_circle!(0, 8, 18),
  hit_circle!(14, 12, 13),
  hit_circle!(-14, 12, 13),
  hit_circle!(0, -12, 16),
  hit_circle!(0, -26, 14),
  hit_circle!(0, -35, 12),
];

const PROWLER_HC: &[HitCircle] = &[
  hit_circle!(0, 11, 25),
  hit_circle!(0, -8, 18),
  hit_circle!(19, 20, 10),
  hit_circle!(-19, 20, 10),
  hit_circle!(0, -20, 14),
];

pub fn hitcircles_for_plane(plane: PlaneType) -> &'static [HitCircle] {
  match plane {
    PlaneType::Predator => PRED_HC,
    PlaneType::Goliath => GOLI_HC,
    PlaneType::Mohawk => MOHAWK_HC,
    PlaneType::Prowler => PROWLER_HC,
    PlaneType::Tornado => TORNADO_HC,
    _ => panic!("got unexpected plane type {:?}", plane),
  }
}
