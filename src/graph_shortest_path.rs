use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
/*
Problem:
Given an graph, return any valid shortest path from the start node to
the destination node.
*/

/*
Iterative Solution:
1. Traverse the graph in breadth-first order, at each node:
    1a. Keep a hashmap mantaining backwards pointers from node -> previous node.
    1a. For each adjacent node:
        1aa. If adjacent node has a previous node already, then continue.
        1ab. If adjacent node is the destination node, traverse the backward pointers
             to reconstruct the path to the destination node.
        1ac. Add backward pointer for adjacent node -> current node.
        1ad. Add adjacent node to queue to explore.
2. If reach end of traversal, no path to destination.

Time: O(N + M), need to visit all nodes in graph, and also consider each edge
    in the graph.
Space: O(N), traversal and backwards pointers is bounded by number of nodes.
*/
pub fn graph_shortest_path<T: Eq + Hash + Clone>(
    adjacency_list: &HashMap<T, Vec<T>>,
    source: T,
    destination: T,
) -> Option<Vec<T>> {
    if source == destination {
        return Some(vec![source]);
    }

    let mut backwards_pointers = HashMap::new();
    backwards_pointers.insert(source.clone(), None);
    let mut to_visit = VecDeque::new();
    to_visit.push_back(source);

    while let Some(node_id) = to_visit.pop_front() {
        if !adjacency_list.contains_key(&node_id) {
            panic!("Graph must have an entry in adjacency_list for each node reachable.");
        }

        for adj_node_id in &adjacency_list[&node_id] {
            // Already visited or in to_visit.
            if backwards_pointers.contains_key(adj_node_id) {
                continue;
            }

            // Found destination, reconstruct path and return.
            if adj_node_id == &destination {
                let mut path = vec![destination];
                let mut current = node_id;
                while let Some(Some(prev)) = backwards_pointers.remove(&current) {
                    path.push(current);
                    current = prev;
                }
                // Flush the last element to the path (the root node does not have a previous).
                path.push(current);
                // Went in backwards order, but returning path from beginning to end.
                path.reverse();
                return Some(path);
            }

            backwards_pointers.insert(adj_node_id.clone(), Some(node_id.clone()));
            to_visit.push_back(adj_node_id.clone());
        }
    }

    // Traversal finished, but no path to destination was found.
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use galvanic_assert::matchers::*;
    use galvanic_assert::*;

    #[test]
    fn works_for_simple() {
        // Graph looks like:
        //     /---(2)----\
        //  (1)          (4)
        //    \---(3)----/
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(1, vec![2, 3]);
        adjacency_table.insert(2, vec![1, 4]);
        adjacency_table.insert(3, vec![1, 4]);
        adjacency_table.insert(4, vec![2, 3]);

        let shortest_path = graph_shortest_path(&adjacency_table, 2, 3);
        assert!(shortest_path.is_some());
        assert_that!(
            &shortest_path.expect("exists"),
            any_of!(eq(vec![2, 1, 3]), eq(vec![2, 4, 3]))
        );
    }

    #[test]
    fn works_for_simple_with_no_path() {
        // Graph looks like:
        //     /---(2)----\
        //  (1)          (4)      (5)
        //    \---(3)----/
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(1, vec![2, 3]);
        adjacency_table.insert(2, vec![1, 4]);
        adjacency_table.insert(3, vec![1, 4]);
        adjacency_table.insert(4, vec![2, 3]);
        adjacency_table.insert(5, vec![]);

        let shortest_path = graph_shortest_path(&adjacency_table, 2, 5);
        assert!(shortest_path.is_none());
    }

    #[test]
    fn works_for_fully_connected() {
        // Graph looks like:
        //     /---(2)----\
        //  (1)-----|----(4)
        //    \---(3)----/
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(1, vec![2, 3, 4]);
        adjacency_table.insert(2, vec![1, 4, 3]);
        adjacency_table.insert(3, vec![1, 4, 2]);
        adjacency_table.insert(4, vec![2, 3, 1]);

        let shortest_path = graph_shortest_path(&adjacency_table, 2, 3);
        assert!(shortest_path.is_some());
        assert_that!(&shortest_path.expect("exists"), eq(vec![2, 3]));
    }

    #[test]
    fn works_for_single_node_going_to_self() {
        // Graph looks like:
        // (1)
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(1, vec![]);

        let shortest_path = graph_shortest_path(&adjacency_table, 1, 1);
        assert!(shortest_path.is_some());
        assert_that!(&shortest_path.expect("exists"), eq(vec![1]));
    }
}
