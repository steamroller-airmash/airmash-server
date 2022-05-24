use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub(crate) struct Aabb {
  pub x: RangeInclusive<f32>,
  pub y: RangeInclusive<f32>,
}

impl Aabb {
  pub fn contains(&self, point: [f32; 2]) -> bool {
    return point[0] >= *self.x.start()
      && point[0] <= *self.x.end()
      && point[1] >= *self.y.start()
      && point[1] <= *self.y.end();
  }

  pub fn clip(&mut self, range: &Self) {
    let min_x = self.x.start().max(*range.x.start());
    let max_x = self.x.end().min(*range.x.end());
    let min_y = self.y.start().max(*range.y.start());
    let max_y = self.y.end().min(*range.y.end());

    *self = Self {
      x: min_x..=max_x,
      y: min_y..=max_y,
    }
  }

  // Grow the range to include the new point
  pub fn expand(&mut self, point: [f32; 2]) {
    self.x = self.x.start().min(point[0])..=self.x.end().max(point[0]);
    self.y = self.y.start().min(point[1])..=self.y.end().max(point[1]);
  }

  pub fn empty() -> Self {
    Self {
      x: 0.0..=0.0,
      y: 0.0..=0.0,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.x.is_empty() || self.y.is_empty()
  }
}

impl Default for Aabb {
  fn default() -> Self {
    Self::empty()
  }
}
