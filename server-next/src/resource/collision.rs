
use airmash_protocol::Vector2;
use hecs::Entity;
use kdtree::KdTree;

def_wrappers! {
  ##[nocopy]
  pub type PlayerPosDb = SpatialTree;
  ##[nocopy]
  pub type PlayerCollideDb = SpatialTree;
  ##[nocopy]
  pub type MissileCollideDb = SpatialTree;
  ##[nocopy]
  pub type Terrain = SpatialTree;
}

#[derive(Copy, Clone, Debug)]
pub struct Entry {
  pub pos: Vector2<f32>,
  pub radius: f32,
  pub entity: Entity
}

impl Entry {
  fn kdtree_func(&self) -> (Vector2<f32>, f32) {
    (self.pos, self.radius)
  }
}

#[derive(Clone, Debug)]
pub struct SpatialTree {
  tree: KdTree<Entry>
}

impl SpatialTree {
  pub fn new() -> Self {
    Self::with_entries(Vec::new())
  }

  pub fn with_entries(entries: Vec<Entry>) -> Self {
    Self {
      tree: KdTree::new(entries, &Entry::kdtree_func)
    }
  }

  pub fn recreate(&mut self, entries: Vec<Entry>) {
    self.tree.rebuild_from(entries, &Entry::kdtree_func);
  }

  pub fn query(&self, pos: Vector2<f32>, rad: f32, out: &mut Vec<Entity>) {
    let mut output = Vec::new();
    self.tree.lookup(pos, rad, &mut output);

    out.reserve(output.len());
    for entry in output {
      out.push(entry.entity);
    }
  }
}
