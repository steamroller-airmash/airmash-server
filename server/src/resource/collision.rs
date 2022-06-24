//! Resources related to collisions and spatial queries.

use hecs::Entity;
use kdtree::{KdTree, Node};

use crate::util::NalgebraExt;
use crate::Vector2;

def_wrappers! {
  #[derive(Default)]
  ##[nocopy]
  pub type PlayerPosDb = SpatialTree;
  #[derive(Default)]
  ##[nocopy]
  pub type PlayerCollideDb = SpatialTree;
  #[derive(Default)]
  ##[nocopy]
  pub type MissileCollideDb = SpatialTree;
  #[derive(Default)]
  ##[nocopy]
  pub type MobCollideDb = SpatialTree;
  ##[nocopy]
  pub type Terrain = SpatialTree;
}

#[derive(Copy, Clone, Debug)]
pub struct Entry {
  pub pos: Vector2,
  pub radius: f32,
  pub entity: Entity,
  pub layer: u16,
}

impl Node for Entry {
  fn position(&self) -> [f32; 2] {
    [self.pos.x, self.pos.y]
  }

  fn radius(&self) -> f32 {
    self.radius
  }
}

pub enum LayerSpec {
  Include(u16),
  Exclude(u16),
  None,
}

#[derive(Clone, Debug, Default)]
pub struct SpatialTree {
  tree: KdTree<Entry>,
}

impl SpatialTree {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_entries(entries: Vec<Entry>) -> Self {
    let mut me = Self::new();
    me.recreate(entries);
    me
  }

  pub fn recreate(&mut self, mut entries: Vec<Entry>) {
    entries.retain(|e| !e.pos.x.is_nan() && !e.pos.y.is_nan() && !e.radius.is_nan());
    self.tree.rebuild_from(&mut entries);
  }

  pub fn contains(&self, pos: Vector2, rad: f32, layer: LayerSpec) -> bool {
    let pos = [pos.x, pos.y];

    match layer {
      LayerSpec::Exclude(layer) => self.tree.within(pos, rad).any(|x| x.layer != layer),
      LayerSpec::Include(layer) => self.tree.within(pos, rad).any(|x| x.layer == layer),
      LayerSpec::None => self
        .tree
        .within(pos, rad)
        .map(|x| x.entity)
        .next()
        .is_some(),
    }
  }

  /// Query all circles within this `SpatialTree` whose centre overlaps the
  /// circle defined by `pos` and `rad`. Generally you will probably want
  /// [`query`] instead.
  ///
  /// This result is then filtered by the `LayerSpec` given in `layer`.
  ///
  /// [`query`]: self::SpatialTree::query
  pub fn query_pos<V: Extend<Entity>>(
    &self,
    pos: Vector2,
    rad: f32,
    layer: LayerSpec,
    out: &mut V,
  ) {
    let r2 = rad * rad;

    let base = self
      .tree
      .within([pos.x, pos.y], rad)
      .filter(|x| (pos - x.pos).norm_squared() <= r2);

    match layer {
      LayerSpec::Exclude(layer) => out.extend(base.filter(|x| x.layer != layer).map(|x| x.entity)),
      LayerSpec::Include(layer) => out.extend(base.filter(|x| x.layer == layer).map(|x| x.entity)),
      LayerSpec::None => out.extend(base.map(|x| x.entity)),
    }
  }

  /// Query all circles within this `SpatialTree` that overlap with the circle
  /// defined by `pos` and `rad`.
  ///
  /// This result is then filtered by the `LayerSpec` given in `layer`.
  pub fn query<V: Extend<Entity>>(&self, pos: Vector2, rad: f32, layer: LayerSpec, out: &mut V) {
    let base = self.tree.within([pos.x, pos.y], rad);

    match layer {
      LayerSpec::Exclude(layer) => out.extend(base.filter(|x| x.layer != layer).map(|x| x.entity)),
      LayerSpec::Include(layer) => out.extend(base.filter(|x| x.layer == layer).map(|x| x.entity)),
      LayerSpec::None => out.extend(base.map(|x| x.entity)),
    }
  }

  pub fn query_all_pairs<'a>(
    &'a self,
    other: &'a SpatialTree,
    out: &mut Vec<(&'a Entry, &'a Entry)>,
  ) {
    if self.tree.len() < other.tree.len() {
      out.extend(self.tree.query_all(&other.tree));
    } else {
      out.extend(other.tree.query_all(&self.tree).map(|(x, y)| (y, x)));
    }
  }
}

impl Default for Terrain {
  fn default() -> Self {
    use crate::consts::TERRAIN;

    let mut entries = Vec::with_capacity(TERRAIN.len());
    for [x, y, r] in TERRAIN {
      entries.push(Entry {
        entity: Entity::from_bits(1 << 32).unwrap(),
        pos: Vector2::new(x as f32, y as f32),
        radius: r as f32,
        layer: 0,
      });
    }

    Terrain(SpatialTree::with_entries(entries))
  }
}
