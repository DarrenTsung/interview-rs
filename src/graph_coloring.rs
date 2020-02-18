use std::collections::{HashMap, HashSet, VecDeque};
/*
Problem:
Given an undirected graph with maximum degree D,
return a coloring of the graph with D + 1 colors.
*/

/*
Naive solution:
1. Choose a random color for a starting node.
2. Use breadth-first traversal, for each node not colored:
    2a. Find all unavailable colors from adj nodes.
    2b. Choose first available color.
    2c. Add newly colored nodes to be visited.

Time complexity: O(N * D)
    - Visit each node and do O(D) operations.
    - But actually we're only traversing each edge for a node at most twice
      for the entire algorithm, so we're actually doing O(N + M).
Space complexity: O(N)
    - At most need to store O(D) per iteration which is bounded by O(N)
    - Breadth-first traversal queue is at most O(N) size.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorId(usize);

pub fn color_undirected_graph_with_max_degree(
    adjacency_table: &HashMap<NodeId, Vec<NodeId>>,
    max_degree: usize,
) -> HashMap<NodeId, ColorId> {
    if adjacency_table.is_empty() {
        return HashMap::new();
    }

    let mut coloring = HashMap::new();

    // Get a random element in the adjacency_table.
    let (&node_id, _) = adjacency_table.iter().next().expect("exists");
    coloring.insert(node_id, ColorId(0));

    // Breadth-first traversal through the graph.
    let mut node_ids = VecDeque::new();
    node_ids.push_back(node_id);
    while let Some(node_id) = node_ids.pop_front() {
        for &adj_node_id in &adjacency_table[&node_id] {
            if coloring.contains_key(&adj_node_id) {
                continue;
            }

            let unavailable_colors = adjacency_table[&adj_node_id]
                .iter()
                .filter(|id| coloring.contains_key(id))
                .map(|id| coloring[id])
                .collect::<HashSet<_>>();

            let available_color_id = {
                let mut available_color_id = None;
                for color_id in 0..=max_degree {
                    if unavailable_colors.contains(&ColorId(color_id)) {
                        continue;
                    }

                    available_color_id = Some(color_id);
                    break;
                }
                available_color_id.expect("always availabe color")
            };
            coloring.insert(adj_node_id, ColorId(available_color_id));

            // Need to visit this adj node as well.
            node_ids.push_back(adj_node_id);
        }
    }

    coloring
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_coloring_valid_for_undirected_graph(
        coloring: &HashMap<NodeId, ColorId>,
        adjacency_table: &HashMap<NodeId, Vec<NodeId>>,
    ) {
        // All colors must exist and all nodes must be colored
        // differently than adjacent nodes.
        for (node, adj_nodes) in adjacency_table {
            let color = coloring[&node];
            for adj_node in adj_nodes {
                let adj_color = coloring[&adj_node];
                assert_ne!(color, adj_color);
            }
        }
    }

    #[test]
    fn works_for_simple() {
        // Graph looks like:
        //     /---(2)----\
        //  (1)          (4)
        //    \---(3)----/
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(NodeId(1), vec![NodeId(2), NodeId(3)]);
        adjacency_table.insert(NodeId(2), vec![NodeId(1), NodeId(4)]);
        adjacency_table.insert(NodeId(3), vec![NodeId(1), NodeId(4)]);
        adjacency_table.insert(NodeId(4), vec![NodeId(2), NodeId(3)]);

        let coloring = color_undirected_graph_with_max_degree(&adjacency_table, 2);
        assert_coloring_valid_for_undirected_graph(&coloring, &adjacency_table);
    }

    #[test]
    fn works_for_fully_connected() {
        // Graph looks like:
        //     /---(2)----\
        //  (1)-----|-----(4)
        //    \---(3)----/
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(NodeId(1), vec![NodeId(2), NodeId(3), NodeId(4)]);
        adjacency_table.insert(NodeId(2), vec![NodeId(1), NodeId(4), NodeId(3)]);
        adjacency_table.insert(NodeId(3), vec![NodeId(1), NodeId(4), NodeId(2)]);
        adjacency_table.insert(NodeId(4), vec![NodeId(2), NodeId(3), NodeId(1)]);

        let coloring = color_undirected_graph_with_max_degree(&adjacency_table, 3);
        assert_coloring_valid_for_undirected_graph(&coloring, &adjacency_table);
    }

    #[test]
    fn works_for_empty_graph() {
        // Graph looks like:
        let adjacency_table = HashMap::new();
        let coloring = color_undirected_graph_with_max_degree(&adjacency_table, 0);
        assert_coloring_valid_for_undirected_graph(&coloring, &adjacency_table);
    }

    #[test]
    fn works_for_single_node() {
        // Graph looks like:
        // (1)
        let mut adjacency_table = HashMap::new();
        adjacency_table.insert(NodeId(1), vec![]);

        let coloring = color_undirected_graph_with_max_degree(&adjacency_table, 0);
        assert_coloring_valid_for_undirected_graph(&coloring, &adjacency_table);
    }
}
