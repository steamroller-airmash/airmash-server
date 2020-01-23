use crate::ecs::Entity;
use crate::util::KdTree;
use crate::{Distance, Position, Vector2};

use fxhash::FxHashSet as HashSet;

use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct PlayerGrid(pub Grid);

impl Deref for PlayerGrid {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Terrain(pub Grid);

impl Deref for Terrain {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Terrain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Terrain {
    fn default() -> Self {
        use crate::data::TERRAIN;

        let hcs: Vec<_> = TERRAIN
            .iter()
            .copied()
            .map(|[x, y, r]| HitCircle {
                pos: Position::new(x as f32, y as f32),
                rad: Distance::new(r as f32),
                layer: 0,
                ent: None,
            })
            .collect();

        Self(Grid::new(hcs))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitCircle {
    pub pos: Position,
    pub rad: Distance,
    pub layer: u16,
    pub ent: Option<Entity>,
}

#[derive(Debug, Clone, Default)]
pub struct Grid {
    tree: KdTree<HitCircle>,
}

impl Grid {
    pub fn new(circles: Vec<HitCircle>) -> Self {
        Self {
            tree: KdTree::new(circles, &pos_accessor),
        }
    }

    pub fn rebuild_from(&mut self, circles: Vec<HitCircle>) {
        self.tree.rebuild_from(circles, &pos_accessor);
    }

    /// Perform a series of collision checks against the grid
    /// and return an iterator over the resulting collisions.
    ///
    /// The result is an iterator of `(external, grid_circle)`.
    pub fn collide<'a, I>(&'a self, b: I) -> impl Iterator<Item = (HitCircle, HitCircle)> + 'a
    where
        I: Iterator<Item = HitCircle> + 'a,
    {
        b.flat_map(move |hc| {
            let (pos, rad) = pos_accessor(&hc);

            self.tree
                .lookup(pos, rad)
                .filter(move |x| x.layer != hc.layer)
                .map(move |x| (hc, *x))
        })
    }

    /// Find all unique entities that can collide with the given hit circle.
    /// This means all hit circles that have an entity and are on a different
    /// layer from `hc`.
    ///
    /// This is meant to be mainly used for visibility calculations. In general
    /// you should be using `collide` instead.
    pub fn entity_collide(&self, hc: HitCircle) -> HashSet<Entity> {
        let (pos, rad) = pos_accessor(&hc);

        self.tree
            .lookup(pos, rad)
            .filter(|x| x.layer != hc.layer)
            .filter_map(|x| x.ent)
            .collect()
    }

    pub fn does_collide(&self, pos: Position, rad: Distance) -> bool {
        self.tree
            .contains_any(Vector2::new(pos.x.inner(), pos.y.inner()), rad.inner())
    }
}

fn pos_accessor(hc: &HitCircle) -> (Vector2<f32>, f32) {
    (
        Vector2::new(hc.pos.x.inner(), hc.pos.y.inner()),
        hc.rad.inner(),
    )
}
