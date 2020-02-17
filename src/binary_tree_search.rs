use ego_binary_tree::{BinaryNodeRef, BinaryTree};
use std::cmp;

/*
Problem: Given a binary tree, return whether the tree is a binary search tree.
*/
pub fn is_binary_search_tree<T: Ord + Clone>(tree: &BinaryTree<T>) -> bool {
    calc_search_bounds(tree.root()).is_some()
}

struct MinMax<T> {
    min: T,
    max: T,
}

/// Returns None if subtree is not search tree.
fn calc_search_bounds<T: Ord + Clone>(node: BinaryNodeRef<T>) -> Option<MinMax<T>> {
    let mut min = node.value().clone();
    let mut max = node.value().clone();

    if let Some(left) = node.left() {
        let bounds = calc_search_bounds(left)?;
        // Not search tree if left sub-tree max value is greater than node value.
        if &bounds.max > node.value() {
            return None;
        }

        min = cmp::min(min, bounds.min);
        max = cmp::max(max, bounds.max);
    }

    if let Some(right) = node.right() {
        let bounds = calc_search_bounds(right)?;
        // Not search tree if right sub-tree min value is less than node value.
        if &bounds.min < node.value() {
            return None;
        }

        min = cmp::min(min, bounds.min);
        max = cmp::max(max, bounds.max);
    }

    Some(MinMax { min, max })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ego_binary_tree::binary_tree;

    #[test]
    fn works_on_search_tree() {
        let tree = binary_tree! {
            3 => {
                left: 2 => {
                    left: 1,
                },
                right: 8 => {
                    left: 6,
                },
            }
        };
        assert_eq!(is_binary_search_tree(&tree), true);
    }

    #[test]
    fn right_subtree_not_search() {
        let tree = binary_tree! {
            3 => {
                left: 2 => {
                    left: 1,
                },
                right: 8 => {
                    left: 6 => {
                        right: 5,
                    },
                },
            }
        };
        assert_eq!(is_binary_search_tree(&tree), false);
    }

    #[test]
    fn not_search_over_multiple_levels() {
        let tree = binary_tree! {
            5 => {
                left: 3 => {
                    left: 1,
                    right: 7,
                },
                right: 8,
            }
        };
        assert_eq!(is_binary_search_tree(&tree), false);
    }

    #[test]
    fn single_leaf_search_tree() {
        let tree = binary_tree! {
            3 => {
                left: 2,
                right: 8,
            }
        };
        assert_eq!(is_binary_search_tree(&tree), true);
    }
}
