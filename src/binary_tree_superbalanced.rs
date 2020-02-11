use ego_binary_tree::{BinaryNodeRef, BinaryTree};
use std::cmp;

pub fn is_superbalanced<T>(tree: &BinaryTree<T>) -> bool {
    let root = tree.root();
    let height_diff = (node_height(root.left()) - node_height(root.right())).abs();
    height_diff <= 1
}

fn node_height<T>(node: Option<BinaryNodeRef<T>>) -> i32 {
    let node = match node {
        None => return 0,
        Some(n) => n,
    };

    cmp::max(node_height(node.left()), node_height(node.right())) + 1
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
