use ego_binary_tree::{BinaryNodeRef, BinaryTree};
use std::collections::VecDeque;
use std::fmt::Debug;

/*
Problem:
Return the 2nd largest value in a binary search tree.
*/
pub fn second_largest_value_in_bst<T: PartialEq + Debug + Clone>(
    tree: &BinaryTree<T>,
) -> Option<T> {
    let naive = naive(tree);
    let better = better(tree);
    let best = best(tree);
    assert_eq!(better, naive);
    assert_eq!(best, naive);
    naive
}

/*
Naive solution:
Traverse tree and store max in circular buffer of size 2.
Return the end element in the circular buffer.

Time complexity: O(N) - traversal takes N time.
Space complexity: O(H) - traversal takes H space (height of tree).
*/
fn naive<T: Debug + Clone>(tree: &BinaryTree<T>) -> Option<T> {
    let mut highest_buffer = VecDeque::with_capacity(2);
    in_order_traversal(Some(tree.root()), &mut |val| {
        highest_buffer.push_back(val.clone());
        if highest_buffer.len() > 2 {
            highest_buffer.pop_front();
        }
    });

    if highest_buffer.len() < 2 {
        None
    } else {
        Some(highest_buffer[0].clone())
    }
}

fn in_order_traversal<T>(node: Option<BinaryNodeRef<T>>, f: &mut impl FnMut(&T)) {
    let node = if let Some(node) = node {
        node
    } else {
        return;
    };

    in_order_traversal(node.left(), f);
    f(node.value());
    in_order_traversal(node.right(), f);
}

/*
Better solution:
Instead of traversing the entire tree, we can recursively go down
the right side and early exit if value is found.

This works easily for different values of N-th largest.

Time complexity: O(H) where H is height of tree.
Space complexity: O(H) due to recursive traversal requirements.
*/
fn better<T: Debug + Clone>(tree: &BinaryTree<T>) -> Option<T> {
    let mut n = 2;
    find_nth_max(Some(tree.root()), &mut n)
}

fn find_nth_max<T: Debug + Clone>(node: Option<BinaryNodeRef<T>>, n: &mut u32) -> Option<T> {
    let node = node?;

    if let Some(found) = find_nth_max(node.right(), n) {
        return Some(found);
    }

    *n -= 1;
    if *n == 0 {
        return Some(node.value().clone());
    }

    if let Some(found) = find_nth_max(node.left(), n) {
        return Some(found);
    }

    None
}

/*
Best solution:
Because we know we're only going to return 2nd highest value,
there are only three possible cases when we fully traverse down the right side.

1. Fully-right node has left child
2. Fully-right node has no left child, has parent
3. Fully-right node has no left child, no parent

1a. If fully-right node has a left child, then 2nd highest node is fully-right child of the left child.
2a. If no left child, 2nd highest node is parent.
3a. If no left child, no parent, then no 2nd highest node.

Time complexity: O(H), need to traverse height of tree to get to maximum node.
Space complexity: O(1)
*/
fn best<T: Debug + Clone>(tree: &BinaryTree<T>) -> Option<T> {
    let (fully_right, fully_right_parent) = {
        let mut parent = None;
        let mut current = tree.root();
        while let Some(right) = current.right() {
            parent = Some(current);
            current = right;
        }
        (current, parent)
    };

    match (fully_right.left(), fully_right_parent) {
        (Some(mut left), _) => {
            let fully_right_of_left_subtree = {
                while let Some(right) = left.right() {
                    left = right;
                }
                left
            };
            Some(fully_right_of_left_subtree)
        }
        (None, Some(parent)) => Some(parent),
        (None, None) => None,
    }
    .map(|n| n.value().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ego_binary_tree::binary_tree;

    #[test]
    fn rightmost_has_parent() {
        let tree = binary_tree! {
            3 => {
                right: 8,
            }
        };
        assert_eq!(second_largest_value_in_bst(&tree), Some(3));
    }

    #[test]
    fn rightmost_has_left_node() {
        let tree = binary_tree! {
            3 => {
                right: 8 => {
                    left: 5,
                },
            }
        };
        assert_eq!(second_largest_value_in_bst(&tree), Some(5));
    }

    #[test]
    fn rightmost_has_left_node_with_more_right() {
        let tree = binary_tree! {
            3 => {
                right: 8 => {
                    left: 5 => {
                        right: 6 => {
                            right: 7,
                        },
                    },
                },
            }
        };
        assert_eq!(second_largest_value_in_bst(&tree), Some(7));
    }

    #[test]
    fn only_root() {
        let tree = BinaryTree::new(3);
        assert_eq!(second_largest_value_in_bst(&tree), None);
    }
}
