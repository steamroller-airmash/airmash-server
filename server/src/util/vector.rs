use ultraviolet::Vec2;

pub trait NalgebraExt {
  fn zeros() -> Self;

  fn norm(&self) -> f32;
  fn norm_squared(&self) -> f32;
}

impl NalgebraExt for Vec2 {
  fn zeros() -> Self {
    Self::zero()
  }

  fn norm(&self) -> f32 {
    self.mag()
  }
  fn norm_squared(&self) -> f32 {
    self.mag_sq()
  }
}
