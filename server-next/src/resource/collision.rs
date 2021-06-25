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
  pub entity: Entity,
  pub layer: u16,
}

impl Entry {
  fn kdtree_func(&self) -> (Vector2<f32>, f32) {
    (self.pos, self.radius)
  }
}

#[derive(Clone, Debug)]
pub struct SpatialTree {
  tree: KdTree<Entry>,
}

impl SpatialTree {
  pub fn new() -> Self {
    Self::with_entries(Vec::new())
  }

  pub fn with_entries(entries: Vec<Entry>) -> Self {
    Self {
      tree: KdTree::new(entries, &Entry::kdtree_func),
    }
  }

  pub fn recreate(&mut self, entries: Vec<Entry>) {
    self.tree.rebuild_from(entries, &Entry::kdtree_func);
  }

  pub fn query(&self, pos: Vector2<f32>, rad: f32, layer: Option<u16>, out: &mut Vec<Entity>) {
    let mut output = Vec::new();
    self.tree.lookup(pos, rad, &mut output);

    if let Some(layer) = layer {
      output.retain(|x| x.layer == layer);
    }

    out.reserve(output.len());
    for entry in output {
      out.push(entry.entity);
    }
  }
}

impl Default for Terrain {
  fn default() -> Self {
    use crate::consts::TERRAIN;

    let mut entries = Vec::with_capacity(TERRAIN.len());
    for [x, y, r] in TERRAIN {
      entries.push(Entry {
        entity: Entity::from_bits(0),
        pos: Vector2::new(x as f32, y as f32),
        radius: r as f32,
        layer: 0,
      });
    }

    Terrain(SpatialTree::with_entries(entries))
  }
}
