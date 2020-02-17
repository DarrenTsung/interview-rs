use ego_binary_tree::{BinaryNodeRef, BinaryTree};

pub fn is_superbalanced<T>(tree: &BinaryTree<T>) -> bool {
    let mut min_leaf_depth = None;
    let mut max_leaf_depth = None;

    // Min and max must be populated by this function.
    populate_min_max_leaf_height(tree.root(), 0, &mut min_leaf_depth, &mut max_leaf_depth);

    let height_diff = max_leaf_depth.expect("exists") - min_leaf_depth.expect("exists");
    height_diff <= 1
}

fn populate_min_max_leaf_height<T>(
    node: BinaryNodeRef<T>,
    depth: u32,
    min_leaf_depth: &mut Option<u32>,
    max_leaf_depth: &mut Option<u32>,
) {
    let left = node.left();
    let right = node.right();
    if left.is_none() && right.is_none() {
        // Found leaf node, populate min / max if necessary.
        if min_leaf_depth.is_none() || depth < min_leaf_depth.expect("exists") {
            *min_leaf_depth = Some(depth);
        }
        if max_leaf_depth.is_none() || depth > max_leaf_depth.expect("exists") {
            *max_leaf_depth = Some(depth);
        }
        return;
    }

    if let Some(left) = left {
        populate_min_max_leaf_height(left, depth + 1, min_leaf_depth, max_leaf_depth);
    }
    if let Some(right) = right {
        populate_min_max_leaf_height(right, depth + 1, min_leaf_depth, max_leaf_depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ego_binary_tree::binary_tree;

    #[test]
    fn works_on_not_superbalanced() {
        let tree = binary_tree! {
            "root" => {
                left: "left",
                right: "right" => {
                    left: "rightleft" => {
                        left: "rightleftleft",
                        right: "rightleftright"
                    },
                    right: "rightright",
                }
            }
        };
        assert_eq!(is_superbalanced(&tree), false);
    }

    #[test]
    fn hidden_leaf() {
        let tree = binary_tree! {
            "root" => {
                left: "l" => {
                    left: "hidden",
                    right: "lr" => {
                        left: "lrl" => {
                            left: "lrll",
                            right: "lrlr",
                        },
                        right: "lrr" => {
                            left: "lrrl",
                            right: "lrrr",
                        },
                    },
                },
                right: "r" => {
                    left: "rl" => {
                        left: "rll" => {
                            left: "rlll",
                            right: "rllr",
                        },
                        right: "rlr" => {
                            left: "rlrl",
                            right: "rlrr",
                        },
                    },
                    right: "rr" => {
                        left: "rrl" => {
                            left: "rrll",
                            right: "rrlr",
                        },
                        right: "rrr" => {
                            left: "rrrl",
                            right: "rrrr",
                        },
                    },
                }
            }
        };
        assert_eq!(is_superbalanced(&tree), false);
    }

    #[test]
    fn works_on_superbalanced() {
        let tree = binary_tree! {
            "root" => {
                left: "left",
                right: "right" => {
                    left: "rightleft",
                    right: "rightright",
                }
            }
        };
        assert_eq!(is_superbalanced(&tree), true);
    }
}
