use std::collections::HashSet;

use kdtree::KdTree;

fn rand_coord(range: f32) -> f32 {
  (rand::random::<f32>() - 0.5) * 2.0 * range
}

fn rand_radius(range: f32) -> f32 {
  rand::random::<f32>() * range
}

struct Node {
  point: [f32; 2],
  radius: f32,
  id: usize,
}

impl kdtree::Node for Node {
  fn position(&self) -> [f32; 2] {
    self.point
  }

  fn radius(&self) -> f32 {
    self.radius
  }
}

#[test]
fn test_random_points() {
  let mut points = vec![];
  let mut expected = 0;
  let coord_range = 1000.0;
  let radius_range = 50.0;

  let query_radius = 500.0;

  for _ in 0..10000 {
    let point = [rand_coord(coord_range), rand_coord(coord_range)];
    let radius = rand_radius(radius_range);

    if (point[0] * point[0] + point[1] * point[1]).sqrt() < query_radius + radius {
      expected += 1;
    }

    points.push((point, radius));
  }

  let tree = KdTree::with_values(&mut points);
  let iter = tree.within([0.0, 0.0], query_radius);
  assert_eq!(iter.count(), expected);

  for &([x, y], r) in tree.within([0.0, 0.0], query_radius) {
    assert!((x * x + y * y).sqrt() < query_radius + r);
  }
}

#[test]
fn test_random_points_2() {
  let mut points = vec![];
  let mut expected = HashSet::new();
  let coord_range = 1000.0;
  let radius_range = 50.0;

  let query_radius = 500.0;

  for id in 0..10000 {
    let point = [rand_coord(coord_range), rand_coord(coord_range)];
    let radius = rand_radius(radius_range);

    if (point[0] * point[0] + point[1] * point[1]).sqrt() < query_radius + radius {
      expected.insert(id);
    }

    points.push(Node { point, radius, id });
  }

  let tree = KdTree::with_values(&mut points);

  for node in tree.within([0.0, 0.0], query_radius) {
    assert!(expected.contains(&node.id));
  }
}
