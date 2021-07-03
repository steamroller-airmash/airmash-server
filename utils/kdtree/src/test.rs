use super::*;

type DefaultNode = ([f32; 2], f32);

#[test]
fn empty_tree_has_no_collisions() {
  let tree: KdTree<DefaultNode> = KdTree::default();
  assert_eq!(tree.within([0.0, 0.0], 1000.0).count(), 0);
}

#[test]
fn points_in_same_location() {
  let mut arr = vec![([0.0, 0.0], 10.0), ([0.0, 0.0], 10.0)];
  let tree = KdTree::with_values(&mut arr);

  let query = tree.within([1.0, 1.0], 0.5);

  assert_eq!(query.count(), 2);
}

#[test]
fn degenerate() {
  let mut values = vec![];
  for _ in 0..100 {
    values.push(([0.0, 0.0], 0.0));
  }
  values.push(([1.0, 1.0], 0.99));

  let tree = KdTree::with_values(&mut values);
  let query = tree.within([0.0, 0.0], 0.0);

  assert_eq!(query.count(), 100);
}

#[test]
fn regression_bad_dir() {
  let mut values = vec![
    ([0.0, 0.0], 0.0),
    ([-1.0, -1.0], 0.0),
    ([4.0, 1.0], 0.0),
    ([2.0, 2.0], 0.0),
  ];

  let tree = KdTree::with_values(&mut values);
  let query = tree.within_aabb(-2.0, 0.5, -2.0, 8.0).collect::<Vec<_>>();

  assert_eq!(query.len(), 2, "{:?}", query);
}
