use std::collections::{HashMap, HashSet};

// Given various subsequences of an array of unique integers, reconstruct the original array:
//
// Example: [1, 3, 5], [1, 3, 9], [9, 5]
// Output : [1, 3, 9, 5]
//
// There may be multiple valid reconstructions of the original array. Return a valid reconstruction.
//
// Example: [1, 3], [5, 3]
// Output : [1, 5, 3] or [5, 1, 3]
pub fn subsequence_reconstruction(subsequences: &[Vec<u32>]) -> Vec<u32> {
    // Iterate through subsequences to create a hashmap containing all directed edges.
    let mut directed_edges = HashMap::new();
    for subsequence in subsequences {
        let mut prev = None;
        for &id in subsequence {
            directed_edges.entry(id).or_insert_with(HashSet::new);

            if let Some(prev) = prev {
                // Previously is always added to the HashMap in a previous iteration^.
                directed_edges.get_mut(&prev).expect("exists").insert(id);
            }

            prev = Some(id);
        }
    }

    // Traverse the directed edges and count number of nodes reachable.
    let mut nodes_reachable = HashMap::new();
    for (&id, _edges) in &directed_edges {
        populate_nodes_reachable(id, &directed_edges, &mut nodes_reachable);
    }

    // Reconstruction is created by ordering the ids by the number of nodes reachable.
    let mut reconstruction = nodes_reachable.keys().cloned().collect::<Vec<_>>();
    reconstruction.sort_by_key(|id| nodes_reachable[id]);
    // We want it to be sorted in descending order.
    reconstruction.reverse();
    reconstruction
}

fn populate_nodes_reachable(
    id: u32,
    directed_edges: &HashMap<u32, HashSet<u32>>,
    nodes_reachable_cache: &mut HashMap<u32, u32>,
) {
    // If already in cache, don't compute.
    if nodes_reachable_cache.contains_key(&id) {
        return;
    }

    let mut nodes_reachable = 0;
    for &other_id in &directed_edges[&id] {
        populate_nodes_reachable(other_id, directed_edges, nodes_reachable_cache);
        nodes_reachable += 1 + nodes_reachable_cache[&other_id];
    }
    nodes_reachable_cache.insert(id, nodes_reachable);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_example() {
        let reconstruction =
            subsequence_reconstruction(&vec![vec![1, 3, 5], vec![1, 3, 9], vec![9, 5]]);
        assert_eq!(reconstruction, vec![1, 3, 9, 5]);
    }

    #[test]
    fn works_on_missing_information() {
        let reconstruction = subsequence_reconstruction(&vec![vec![1, 3], vec![5, 3]]);
        assert_eq!(reconstruction.len(), 3);
        let index1 = reconstruction.iter().position(|&x| x == 1).expect("in");
        let index3 = reconstruction.iter().position(|&x| x == 3).expect("in");
        let index5 = reconstruction.iter().position(|&x| x == 5).expect("in");
        assert!(index1 < index3);
        assert!(index5 < index3);
    }

    #[test]
    fn works_on_more_complicated() {
        let reconstruction =
            subsequence_reconstruction(&vec![vec![5, 0], vec![5, 2, 3, 1], vec![4, 0], vec![4, 1]]);
        assert_eq!(reconstruction.len(), 6);
        let index0 = reconstruction.iter().position(|&x| x == 0).expect("in");
        let index1 = reconstruction.iter().position(|&x| x == 1).expect("in");
        let index2 = reconstruction.iter().position(|&x| x == 2).expect("in");
        let index3 = reconstruction.iter().position(|&x| x == 3).expect("in");
        let index4 = reconstruction.iter().position(|&x| x == 4).expect("in");
        let index5 = reconstruction.iter().position(|&x| x == 5).expect("in");
        assert!(index5 < index0);
        assert!(index5 < index2);
        assert!(index5 < index3);
        assert!(index5 < index1);

        assert!(index2 < index3);
        assert!(index2 < index1);

        assert!(index4 < index0);
        assert!(index4 < index1);
    }
}

/*
Notes:
We can view this as a graph problem where [1, 3, 5] means that 1 -> 3 -> 5 and we must
return a topological sort of the graph.

The unique integers aspect of the problem is there so each integer is a "nodeId".

We can scan the subsequences to create a hashmap containing the directed edges.
{ 1: [3], 5: [3], 3: [], }

Once we've fully created the map, we can then traverse it and create a map
containing the count of nodes reachable from the current node.

{ 1: 1, 5: 1, 3: 0 }

Then a valid reconstruction is obtained by sorting the keys by their value in the hashmap (descending).
[1, 5, 3]

---

Let's run this on the original example:
Directed edges map: {
    1: [3],
    3: [5, 9],
    5: [],
    9: [5],
}

Nodes reachable map: {
    1: 3,
    3: 2,
    5: 0,
    9: 1,
}

Therefore we can return [1, 3, 9, 5].

---

What about a more complicated example like:
[5, 0], [5, 2, 3, 1], [4, 0], [4, 1]

Directed edges map: {
    5: [0, 2],
    0: [],
    2: [3],
    3: [1],
    1: [],
    4: [0, 1],
}

Nodes reachable map: {
    0: 0,
    1: 0,
    3: 1,
    2: 2,
    4: 2,
    5: 3,
}

We return: [5, 4, 2, 3, 1, 0].
*/
