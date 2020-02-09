use ego_tree::{NodeRef, Tree};
use std::collections::VecDeque;

/// Returns true if element is in the tree. Uses iterative breadth-first search.
pub fn breadth_first_find_iterative<T: PartialEq>(tree: &Tree<T>, elem: &T) -> bool {
    let mut nodes = VecDeque::new();
    nodes.push_front(tree.root());

    while let Some(node) = nodes.pop_front() {
        if node.value() == elem {
            return true;
        }

        for child in node.children() {
            nodes.push_back(child);
        }
    }

    false
}

/// Returns true if element is in the tree. Uses recursive depth-first search.
pub fn depth_first_find_recursive<T: PartialEq>(tree: &Tree<T>, elem: &T) -> bool {
    depth_first_find_recursive_helper(tree.root(), elem)
}

fn depth_first_find_recursive_helper<T: PartialEq>(node: NodeRef<T>, elem: &T) -> bool {
    if node.value() == elem {
        return true;
    }

    for child in node.children() {
        let found_in_child_tree = depth_first_find_recursive_helper(child, elem);
        if found_in_child_tree {
            return true;
        }
    }

    false
}

pub fn depth_first_find_iterative<T: PartialEq>(tree: &Tree<T>, elem: &T) -> bool {
    let mut nodes = VecDeque::new();
    nodes.push_front(tree.root());

    while let Some(node) = nodes.pop_front() {
        if node.value() == elem {
            return true;
        }

        for child in node.children() {
            nodes.push_front(child);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use ego_tree::tree;

    macro_rules! find_tests {
        ($test_name:ident, $find_fn:expr) => {
            #[test]
            fn $test_name() {
                let tree = tree!('a' => { 'b', 'c' => { 'd', 'e' } });
                assert_eq!($find_fn(&tree, &'a'), true);
                assert_eq!($find_fn(&tree, &'b'), true);
                assert_eq!($find_fn(&tree, &'c'), true);
                assert_eq!($find_fn(&tree, &'d'), true);
                assert_eq!($find_fn(&tree, &'e'), true);
                assert_eq!($find_fn(&tree, &'f'), false);
            }
        };
    }

    find_tests!(bfs_iter_works, breadth_first_find_iterative);
    find_tests!(dfs_iter_works, depth_first_find_iterative);
    find_tests!(dfs_recur_works, depth_first_find_recursive);
}
