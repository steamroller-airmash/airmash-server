use std::fmt;
use std::fmt::Debug;
use std::mem::MaybeUninit;

use crate::aabb::*;
use crate::Node;

fn split_around_mut<T>(slice: &mut [T], index: usize) -> (&mut [T], &mut T, &mut [T]) {
  let (front, rest) = slice.split_at_mut(index);
  let (mid, rest) = rest.split_first_mut().unwrap();

  (front, mid, rest)
}

fn level_direction(level: usize) -> usize {
  level % 2
}

/// # Invariants
/// - All nodes in the left tree have pos[dir] <= value.pos[dir]
/// - All nodes in the right tree have pos[dir] >= value.pos[dir]
#[derive(Clone, Debug)]
struct Entry<T> {
  value: T,
  left: Option<usize>,
  right: Option<usize>,
}

impl<T> Entry<T> {
  fn new(value: T) -> Self {
    Self {
      value,
      left: None,
      right: None,
    }
  }
}

#[derive(Clone)]
pub struct KdTree<T> {
  entries: Vec<Entry<T>>,
  extents: Aabb,
  max_radius: f32,
}

impl<T: Node> KdTree<T> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn len(&self) -> usize {
    self.entries.len()
  }
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Build this KdTree from a vector of elements.
  ///
  /// Internally this just creates an empty tree and uses `rebuild_from`.
  pub fn with_values(values: &mut Vec<T>) -> Self {
    let mut me = Self::default();
    me.rebuild_from(values);
    me
  }

  /// Rebuild this KdTree from an existing vector of elements.
  ///
  /// This will delete all existing elements within the tree and replace them
  /// with a tree that is built from the values in the provided vector.
  ///
  /// # Panics
  /// This function will panic if the positions of any of the values are NaN.
  ///
  /// If a panic occurs then some elements may be forgotten.
  pub fn rebuild_from(&mut self, values: &mut Vec<T>) {
    self.entries.clear();
    self.max_radius = 0.0;

    for val in values.iter() {
      assert!(!val.position()[0].is_nan(), "KdTree entry position was NaN");
      assert!(!val.position()[1].is_nan(), "KdTree entry position was NaN");
      assert!(!val.radius().is_nan(), "KdTree entry radius was NaN");

      self.extents.expand(val.position());
      self.max_radius = self.max_radius.max(val.radius());
    }

    unsafe {
      let len = values.len();
      values.set_len(0);
      self.build_from_impl(
        std::slice::from_raw_parts_mut(values.as_mut_ptr() as _, len),
        0,
      );
    }
  }

  /// Find all circles that overlap with the provided query circle.
  pub fn within(&self, point: [f32; 2], radius: f32) -> impl Iterator<Item = &T> {
    assert!(radius >= 0.0);

    let [px, py] = point;
    let r = radius + self.max_radius;

    let bounds = Aabb {
      x: px - r..=px + r,
      y: py - r..=py + r,
    };

    // Do an AABB query and then filter out all circles that do not overlap.
    WithinIterator::new(self, bounds).filter(move |x| {
      let [vx, vy] = x.position();
      let r = x.radius() + radius;
      let dx = vx - px;
      let dy = vy - py;

      dx * dx + dy * dy <= r * r
    })
  }

  pub fn within_aabb(
    &self,
    x_lo: f32,
    x_hi: f32,
    y_lo: f32,
    y_hi: f32,
  ) -> impl Iterator<Item = &T> {
    WithinIterator::new(
      self,
      Aabb {
        x: x_lo..=x_hi,
        y: y_lo..=y_hi,
      },
    )
  }

  /// Efficiently find all overlapping circles between the two trees.
  pub fn query_all<'c, 'a: 'c, 'b: 'c, U: Node>(
    &'a self,
    other: &'b KdTree<U>,
  ) -> impl Iterator<Item = (&'a T, &'b U)> + 'c {
    self.entries.iter().flat_map(move |e| {
      other
        .within(e.value.position(), e.value.radius())
        .map(move |y| (&e.value, y))
    })
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.entries.iter().map(|x| &x.value)
  }
}

impl<T: Node> KdTree<T> {
  /// # Safety
  /// All values within `values` must start off initialized. They will all be
  /// moved out of the provided slice unless a panic occurs. In that case the
  /// state of each value is unknown.
  unsafe fn build_from_impl(
    &mut self,
    values: &mut [MaybeUninit<T>],
    level: usize,
  ) -> Option<usize> {
    if values.is_empty() {
      return None;
    }

    let median = values.len() / 2;
    let slice: &mut [T] = values.align_to_mut().1;
    let dir = level_direction(level);

    pdqselect::select_by(slice, median, |a, b| {
      let ap = a.position()[dir];
      let bp = b.position()[dir];

      f32::partial_cmp(&ap, &bp).expect("KdTree node had NaN as position")
    });

    let (front, mid, back) = split_around_mut(values, median);

    let index = self.entries.len();
    self.entries.push(Entry::new(std::ptr::read(mid.as_ptr())));

    let left = self.build_from_impl(front, level + 1);
    let right = self.build_from_impl(back, level + 1);

    self.entries[index].left = left;
    self.entries[index].right = right;

    Some(index)
  }
}

impl<T> Default for KdTree<T> {
  fn default() -> Self {
    Self {
      entries: Vec::new(),
      extents: Aabb::empty(),
      max_radius: 0.0,
    }
  }
}

impl<T: Debug> Debug for KdTree<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list()
      .entries(self.entries.iter().map(|x| &x.value))
      .finish()
  }
}

struct WithinIterator<'a, T> {
  bounds: Aabb,
  stack: Vec<(usize, usize)>,
  tree: &'a KdTree<T>,
}

impl<'a, T: Node> WithinIterator<'a, T> {
  pub fn new(tree: &'a KdTree<T>, bounds: Aabb) -> Self {
    let mut stack = Vec::with_capacity(1);
    // bounds.clip(&tree.extents);

    if !tree.is_empty() && !bounds.is_empty() {
      stack.push((0, 0));
    }

    Self {
      bounds,
      stack,
      tree,
    }
  }
}

impl<'a, T: Node> Iterator for WithinIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let (index, level) = self.stack.pop()?;
      let entry = &self.tree.entries[index];
      let dir = level_direction(level);

      let full_pos = entry.value.position();

      let pos = full_pos[dir];
      let bound = match dir {
        0 => self.bounds.x.clone(),
        _ => self.bounds.y.clone(),
      };

      if pos >= *bound.start() {
        if let Some(index) = entry.left {
          self.stack.push((index, level + 1));
        }
      }

      if pos <= *bound.end() {
        if let Some(index) = entry.right {
          self.stack.push((index, level + 1));
        }
      }

      if self.bounds.contains(full_pos) {
        return Some(&entry.value);
      }
    }
  }
}
