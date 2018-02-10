#![crate_type = "lib"]

//! You can represent a binary tree in a simple flat list using the following
//! structure:
//!
//!
//! ```
//! #       3
//! #   1       5
//! # 0   2   4   6  ...
//! ```
//!
//! This module exposes a series of functions to help you build and maintain
//! this data structure.

/// Returns the flat-tree of the tree node at the specified depth and offset.
///
/// ```
/// assert_eq!(flat_tree::index(0, 0), 0);
/// assert_eq!(flat_tree::index(0, 1), 2);
/// assert_eq!(flat_tree::index(0, 2), 4);
/// ```
pub fn index(depth: u64, offset: u64) -> u64 {
  return (offset << depth + 1) | ((1 << depth) - 1);
}

/// Returns the depth of a node.
///
/// ```
/// assert_eq!(flat_tree::depth(0), 0);
/// assert_eq!(flat_tree::depth(1), 1);
/// assert_eq!(flat_tree::depth(2), 0);
/// assert_eq!(flat_tree::depth(3), 2);
/// assert_eq!(flat_tree::depth(4), 0);
/// ```
pub fn depth(i: u64) -> u64 {
  let mut depth = 0;
  let mut i = i;
  while i & 1 != 0 {
    i >>= 1;
    depth += 1;
  }
  return depth;
}

/// Returns the offset of a node with a depth.
pub fn offset_with_depth(i: u64, depth: u64) -> u64 {
  return i >> (depth + 1);
}

/// Returns the offset of a node.
///
/// ```
/// assert_eq!(flat_tree::offset(0), 0);
/// assert_eq!(flat_tree::offset(1), 0);
/// assert_eq!(flat_tree::offset(2), 1);
/// assert_eq!(flat_tree::offset(3), 0);
/// assert_eq!(flat_tree::offset(4), 2);
/// ```
pub fn offset(i: u64) -> u64 {
  return offset_with_depth(i, depth(i));
}

/// Returns the parent of a node with a depth.
pub fn parent_with_depth(i: u64, depth: u64) -> u64 {
  return index(depth + 1, offset_with_depth(i, depth) >> 1);
}

/// Returns the parent of a node.
///
/// ```
/// assert_eq!(flat_tree::index(1, 0), 1);
/// assert_eq!(flat_tree::index(1, 1), 5);
/// assert_eq!(flat_tree::index(2, 0), 3);
///
/// assert_eq!(flat_tree::parent(0), 1);
/// assert_eq!(flat_tree::parent(2), 1);
/// assert_eq!(flat_tree::parent(1), 3);
/// ```
pub fn parent(i: u64) -> u64 {
  return parent_with_depth(i, depth(i));
}

/// Returns the sibling of a node with a depth.
pub fn sibling_with_depth(i: u64, depth: u64) -> u64 {
  return index(depth, offset(i) ^ 1);
}

/// Returns the sibling of a node.
///
/// ```
/// assert_eq!(flat_tree::sibling(0), 2);
/// assert_eq!(flat_tree::sibling(2), 0);
/// assert_eq!(flat_tree::sibling(1), 5);
/// assert_eq!(flat_tree::sibling(5), 1);
/// ```
pub fn sibling(i: u64) -> u64 {
  return sibling_with_depth(i, depth(i));
}

/// Returns the parent's sibling, of a node, with a depth.
pub fn uncle_with_depth(i: u64, depth: u64) -> u64 {
  return sibling_with_depth(parent_with_depth(i, depth), depth + 1);
}

/// Returns the parent's sibling, of a node.
pub fn uncle(i: u64) -> u64 {
  return uncle_with_depth(i, depth(i));
}

/// Returns both children of a node, with a depth.
pub fn children_with_depth(i: u64, depth: u64) -> (u64, u64) {
  if depth == 0 {
    return (i, i);
  }

  let off = offset_with_depth(i, depth) >> 1;
  return (index(depth - 1, off), index(depth - 1, off + 1));
}

/// Returns both children of a node.
///
/// ```
/// assert_eq!(flat_tree::children(0), (0, 0));
/// assert_eq!(flat_tree::children(1), (0, 2));
/// assert_eq!(flat_tree::children(3), (1, 5));
/// ```
pub fn children(i: u64) -> (u64, u64) {
  return children_with_depth(i, depth(i));
}

/// Returns only the left child of a node, with a depth
pub fn left_child_with_depth(i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return index(depth - 1, offset_with_depth(i, depth) << 1);
}

/// Returns only the left child of a node.
///
/// ```
/// assert_eq!(flat_tree::left_child(0), 0);
/// assert_eq!(flat_tree::left_child(1), 0);
/// assert_eq!(flat_tree::left_child(3), 1);
/// ```
pub fn left_child(i: u64) -> u64 {
  return left_child_with_depth(i, depth(i));
}

/// Returns only the left child of a node, with a depth.
pub fn right_child_with_depth(i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return index(depth - 1, (offset_with_depth(i, depth) << 1) + 1);
}

/// Returns only the left child of a node.
///
/// ```
/// assert_eq!(flat_tree::right_child(0), 0);
/// assert_eq!(flat_tree::right_child(1), 2);
/// assert_eq!(flat_tree::right_child(3), 5);
/// ```
pub fn right_child(i: u64) -> u64 {
  return right_child_with_depth(i, depth(i));
}

/// Returns the right most node in the tree that the node spans, with a depth.
pub fn right_span_with_depth(i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return (offset_with_depth(i, depth) + 1) * (2 << depth) - 2;
}

/// Returns the right most node in the tree that the node spans.
///
/// ```
/// assert_eq!(flat_tree::right_span(0), 0);
/// assert_eq!(flat_tree::right_span(1), 2);
/// assert_eq!(flat_tree::right_span(3), 6);
/// assert_eq!(flat_tree::right_span(23), 30);
/// assert_eq!(flat_tree::right_span(27), 30);
/// ```
pub fn right_span(i: u64) -> u64 {
  return right_span_with_depth(i, depth(i));
}

/// Returns the left most node in the tree that the node spans, with a depth.
pub fn left_span_with_depth(i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return offset_with_depth(i, depth) * (2 << depth);
}

/// Returns the left most node in the tree that it spans.
///
/// ```
/// assert_eq!(flat_tree::left_span(0), 0);
/// assert_eq!(flat_tree::left_span(1), 0);
/// assert_eq!(flat_tree::left_span(3), 0);
/// assert_eq!(flat_tree::left_span(23), 16);
/// assert_eq!(flat_tree::left_span(27), 24);
/// ```
pub fn left_span(i: u64) -> u64 {
  return left_span_with_depth(i, depth(i));
}

/// Returns the left and right most nodes in the tree that the node spans, with
/// a depth.
pub fn spans_with_depth(i: u64, depth: u64) -> (u64, u64) {
  return (
    left_span_with_depth(i, depth),
    right_span_with_depth(i, depth),
  );
}

/// Returns the left and right most nodes in the tree that the node spans.
///
/// ```
/// assert_eq!(flat_tree::spans(0), (0, 0));
/// assert_eq!(flat_tree::spans(1), (0, 2));
/// assert_eq!(flat_tree::spans(3), (0, 6));
/// assert_eq!(flat_tree::spans(23), (16, 30));
/// assert_eq!(flat_tree::spans(27), (24, 30));
/// ```
pub fn spans(i: u64) -> (u64, u64) {
  return spans_with_depth(i, depth(i));
}

/// Returns how many nodes are in the tree that the node spans, with a depth.
pub fn count_with_depth(_: u64, depth: u64) -> u64 {
  return (2 << depth) - 1;
}

/// Returns how many nodes are in the tree that the node spans.
///
/// ```
/// assert_eq!(flat_tree::count(0), 1);
/// assert_eq!(flat_tree::count(1), 3);
/// assert_eq!(flat_tree::count(3), 7);
/// assert_eq!(flat_tree::count(5), 3);
/// assert_eq!(flat_tree::count(23), 15);
/// assert_eq!(flat_tree::count(27), 7);
/// ```
pub fn count(i: u64) -> u64 {
  return count_with_depth(i, depth(i));
}

/// Returns all the previous fully rooted trees before the node.
///
/// ```
/// assert_eq!(flat_tree::full_roots(0), []);
/// assert_eq!(flat_tree::full_roots(2), [0]);
/// assert_eq!(flat_tree::full_roots(8), [3]);
/// assert_eq!(flat_tree::full_roots(20), [7, 17]);
/// assert_eq!(flat_tree::full_roots(18), [7, 16]);
/// assert_eq!(flat_tree::full_roots(16), [7]);
/// ```
pub fn full_roots(i: u64) -> Vec<u64> {
  let mut result = Vec::with_capacity(64);

  if i & 1 != 0 {
    return result;
  }

  let mut tmp = i >> 1;
  let mut offset = 0;
  let mut factor = 1;

  loop {
    if tmp == 0 {
      return result;
    }
    while factor * 2 <= tmp {
      factor *= 2;
    }
    result.push(offset + factor - 1);
    offset += 2 * factor;
    tmp -= factor;
    factor = 1;
  }
}

#[test]
fn test_parent_gt_int32() {
  assert_eq!(parent(10000000000), 10000000001);
}

#[test]
fn test_child_to_parent_to_child() {
  let mut child = 0;
  for _ in 0..50 {
    child = parent(child)
  }
  assert_eq!(child, 1125899906842623);
  for _ in 0..50 {
    child = left_child(child)
  }
  assert_eq!(child, 0);
}
