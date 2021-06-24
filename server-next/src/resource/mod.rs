
use std::time::Instant;
use kdtree::KdTree;
use hecs::Entity;

mod config;
pub mod collision;

pub use self::config::*;

def_wrappers! {
  pub type LastFrame = Instant;
  pub type ThisFrame = Instant;
  pub type StartTime = Instant;

  ##[nocopy]
  pub type Terrain = KdTree<Entity>;
}
