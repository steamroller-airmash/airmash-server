#![allow(dead_code)]

mod aabb;
mod kdtree;

#[cfg(test)]
mod test;

pub use self::kdtree::KdTree;

pub trait Node {
  fn position(&self) -> [f32; 2];
  fn radius(&self) -> f32;
}

impl Node for ([f32; 2], f32) {
  fn position(&self) -> [f32; 2] {
    self.0
  }

  fn radius(&self) -> f32 {
    self.1
  }
}
