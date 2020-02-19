// Find the duplicate given the following constraints:
// 1. The integers are in the range 1..n
// 2. The list has a length of n+1
pub fn find_duplicate(values: &Vec<i32>) -> i32 {
    let binary_search_value = find_duplicate_binary_search(values);
    let graph_value = find_duplicate_graph(values);
    assert_eq!(binary_search_value, graph_value);
    binary_search_value
}

fn find_duplicate_binary_search(values: &Vec<i32>) -> i32 {
    let n = values.len() - 1;

    let mut lower = 1;
    let mut upper = n as i32;

    loop {
        let mid = (lower + upper) / 2;
        let mut in_lower = 0;
        let mut in_upper = 0;
        let mut in_mid = 0;
        for &value in values {
            if lower <= value && value < mid {
                in_lower += 1;
            } else if mid < value && value <= upper {
                in_upper += 1;
            } else if mid == value {
                in_mid += 1;
            }
        }

        if in_mid > 1 {
            return mid;
        }

        if in_upper > in_lower {
            lower = mid + 1;
        } else {
            upper = mid;
        }
    }
}

/*
Graph solution:
To get a better time complexity, we can treat each element in the array
as a pointer to the position specified by the value (See the solution
here for more details: https://www.interviewcake.com/question/python/find-duplicate-optimize-for-space-beast-mode?course=fc1&section=trees-graphs).

Because the bounds of the positions specified by the values is one less than
the length of the array, we can start from the last element in the array (as there
are no values to that position).

The duplicate value is also the position at the beginning of the cycle.

The solution is as follows:
1. We can start from the head of the graph (the last element) and advance N
   steps to ensure that we're in the cycle that must exist.
2. We can then count the size of the cycle by advancing until we reach the same position.
3. We can then find the beginning of the cycle by keeping two pointers, one at the head and
   one which is CYCLE_SIZE positions ahead of the head. By advancing these pointers together,
   we can find the beginning of the cycle when they are at the same position.
4. The position of the beginning of the cycle is a duplicate value. (Return this).

Time: O(N) - Ending up in the cycle takes N ops, finding the cycle is bounded by N, and
   finding the beginning of the cycle is bounded by N.
Space: O(1) - We end up keeping a constant number of pointers / counters.
*/
fn find_duplicate_graph(values: &Vec<i32>) -> i32 {
    let head_index = values.len() - 1;

    // Follows the value at the current index to arrive to a new index.
    // Casting is okay since all values must be in range 1..n.
    let advance_index = |index| (values[index] - 1) as usize;

    // Walk through the graph N times to ensure we're in the cycle.
    let index_in_the_cycle = {
        let mut curr = head_index;
        for _ in 0..values.len() {
            curr = advance_index(curr);
        }
        curr
    };

    // Find size of cycle by walking until reaching the same index again.
    let size_of_cycle = {
        let mut next = advance_index(index_in_the_cycle);
        let mut count = 1;
        while next != index_in_the_cycle {
            next = advance_index(next);
            count += 1;
        }
        count
    };
    debug_assert!(size_of_cycle > 0);

    // Walk two pointers together, one is size_of_cycle ahead.
    // When the two pointers are at the same position, we've found the
    // beginning of the cycle.
    let mut behind = head_index;
    let mut ahead = {
        let mut i = head_index;
        for _ in 0..size_of_cycle {
            i = advance_index(i);
        }
        i
    };

    while behind != ahead {
        behind = advance_index(behind);
        ahead = advance_index(ahead);
    }

    // Now that we're at the beginning of the cycle, the
    // duplicate value is the "position" (index + 1).
    (behind + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(find_duplicate(&vec![1, 3, 3, 2]), 3);
        assert_eq!(find_duplicate(&vec![1, 2, 3, 2]), 2);
        assert_eq!(find_duplicate(&vec![2, 3, 1, 1]), 1);
    }

    #[test]
    fn ends_in_cycle_of_size_one() {
        assert_eq!(find_duplicate(&vec![1, 2, 3, 2]), 2);
    }

    #[test]
    fn more_examples() {
        assert_eq!(find_duplicate(&vec![3, 4, 2, 3, 1, 5]), 3);
        assert_eq!(find_duplicate(&vec![3, 1, 2, 2]), 2);
        assert_eq!(find_duplicate(&vec![4, 3, 1, 1, 4]), 4);
    }
}
