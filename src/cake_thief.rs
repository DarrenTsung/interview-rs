/*
Problem:
Given a list of cake types (a cake type has a weight and a monetary value), of which you can take
an unlimited amount, what is the maximum value you can store inside a bag of size S?

Example:
cake_types = [(7, 160), (3, 90), (2, 15)]
capacity    = 20

(6 of middle cake and 1 of last type of cake)
max_bag_value(cake_types, capacity) = 555
*/

/*
Notes:
Thinking about sorting the list of cake tuples, perhaps by (value / weight).
For the example:
(7, 160) = 160/7 = 22.85
(3, 90) = 90/3 = 30
(2, 15) = 15/2 = 7.5
The greedy approach would work in this case because it would take from middle
until there was only 1 left for the last cake.

However, the greedy approach does not work in this case:
cake_types = [(3, 6), (2, 3.9), (1, 1)]
capacity = 4

Even though (3, 6) has a better price to weight ratio, remaining capacity can only fit (1, 1),
it's better to take two (2, 3.9) for a value of 7.8.

---

A bottoms-up approach might be more appropriate here. We can calculate the maximum value
from the answer of the same problem with different capacity.

Namely, for the example that does not work for greedy above:
cake_types = [(3, 6), (2, 3.9), (1, 1)]
capacity = 4

f(4) = max(f(1) + 6, f(2) + 3.9, f(3) + 1).
This recursive solution will duplicate work when called since the order of which cakes
we take doesn't matter. Illustrated, this looks like:
f(4)
    choose 3 -> f(1)
        choose 1 -> f(0) => [3, 1]
    choose 2 -> f(2)
        choose 2 -> f(0) => [2, 2]
        choose 1 -> f(1)
            choose 1 -> f(0) => [2, 1, 1]
    choose 1 -> f(3)
        choose 3 -> f(0) => [1, 3]
        choose 2 -> f(1)
            choose 1 -> f(0) => [1, 2, 1]
        choose 1 -> f(2)
            choose 1 -> f(1)
                choose 1 -> f(0) => [1, 1, 1, 1]

However, even if we remove result duplication above, we are still doing extra work
to calculate things like f(2) multiple times. We can memonize this work in the function.

Recursive psuedo-code:
fn max_bag_value(cake_types: &[CakeType], bag_size: u32, cache: &mut HashMap<u32, f32>) -> f32 {
    if cache.contains_key(bag_size) {
        return cache[bag_size];
    }

    let mut max_value = 0;
    for i in 0..cake_types.len() {
        let current_type = cake_types[i];
        // Handle edge case where cakes can have no weight.
        if current_type.weight == 0 {
            if current_type.value > 0 {
                // Set max value to infinity if a cake has value and no weight.
                max_value = std::f32::INFINITY;
                break;
            } else {
                // Ignore cakes that have no weight + no value.
                continue;
            }
        }

        if current_type.weight > bag_size {
            continue;
        }

        let new_size = bag_size - current_type.weight;
        let inner_max_value = max_bag_value(cake_types, new_size, cache) + current_type.value;
        max_value = cmp::max(max_value, inner_max_value);
    }
    cache.insert(bag_size, max_value);
    max_value
}

Time complexity: O(N * M) where N is the bag size given and M is the number of cake types.
    - We calculate the max_bag_value for at max N values (0..bag_size) and each calculation
      requires looking through each cake type.
Space complexity: O(N)
    - Need O(N) for the cache.
    - Need O(N) for the stack size (worst case we remove 1 from bag size).
*/
use std::collections::HashMap;

#[derive(Debug)]
pub struct CakeType {
    weight: u32,
    value: f32,
}

pub fn max_bag_value(cake_types: &[CakeType], bag_size: u32) -> f32 {
    let mut recursive_cache = HashMap::new();
    let recursive = recursive(cake_types, bag_size, &mut recursive_cache);
    let bottoms_up = bottoms_up(cake_types, bag_size);
    assert_eq!(bottoms_up, recursive);
    recursive
}

fn recursive(cake_types: &[CakeType], bag_size: u32, cache: &mut HashMap<u32, f32>) -> f32 {
    if cache.contains_key(&bag_size) {
        return cache[&bag_size];
    }

    let mut max_value = 0.0;
    for i in 0..cake_types.len() {
        let current_type = &cake_types[i];
        // Handle edge case where cakes can have no weight.
        if current_type.weight == 0 {
            if current_type.value > 0.0 {
                // Set max value to infinity if a cake has value and no weight.
                max_value = std::f32::INFINITY;
                break;
            } else {
                // Ignore cakes that have no weight + no value / negative value.
                continue;
            }
        }

        if current_type.weight > bag_size {
            continue;
        }

        let new_size = bag_size - current_type.weight;
        let inner_max_value = recursive(cake_types, new_size, cache) + current_type.value;
        if inner_max_value > max_value {
            max_value = inner_max_value;
        }
    }
    cache.insert(bag_size, max_value);
    max_value
}

/*
For a bottoms up solution, we can create an array of size N which stores max_bag_value
at each index for that bag size. We can go through each index and calculate the max_bag_value
by going through each of the cake types.
*/
fn bottoms_up(cake_types: &[CakeType], bag_size: u32) -> f32 {
    let mut max_bag_values = vec![];
    for curr_bag_size in 0..=bag_size {
        let mut max_bag_value = 0.0;
        for cake_type in cake_types {
            // Handle cakes with no weight edge case (if value, return INFINITY, otherwise ignore cake).
            if cake_type.weight == 0 {
                if cake_type.value > 0.0 {
                    return std::f32::INFINITY;
                } else {
                    continue;
                }
            }

            // Can't hold cake in the current bag size.
            if cake_type.weight > curr_bag_size {
                continue;
            }

            let other_bag_value =
                max_bag_values[(curr_bag_size - cake_type.weight) as usize] + cake_type.value;
            if other_bag_value > max_bag_value {
                max_bag_value = other_bag_value;
            }
        }
        max_bag_values.push(max_bag_value);
    }

    dbg!(&max_bag_values);
    max_bag_values[bag_size as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_for_example() {
        let cake_types = vec![
            CakeType {
                weight: 7,
                value: 160.0,
            },
            CakeType {
                weight: 3,
                value: 90.0,
            },
            CakeType {
                weight: 2,
                value: 15.0,
            },
        ];

        // (6 of middle cake and 1 of last type of cake)
        assert_eq!(max_bag_value(&cake_types, 20), 555.0);
    }

    #[test]
    fn works_when_greedy_approach_fails() {
        let cake_types = vec![
            CakeType {
                weight: 3,
                value: 6.0,
            },
            CakeType {
                weight: 2,
                value: 3.9,
            },
            CakeType {
                weight: 1,
                value: 1.0,
            },
        ];

        // 2 of the middle cake
        assert_eq!(max_bag_value(&cake_types, 4), 7.8);
    }

    #[test]
    fn works_when_cakes_have_weight_of_zero() {
        let cake_types = vec![
            CakeType {
                weight: 0,
                value: 6.0,
            },
            CakeType {
                weight: 1,
                value: 1.0,
            },
        ];

        assert_eq!(max_bag_value(&cake_types, 4), std::f32::INFINITY);
    }

    #[test]
    fn works_when_cakes_have_weight_of_zero_and_value_of_zero() {
        let cake_types = vec![
            CakeType {
                weight: 1,
                value: 1.0,
            },
            CakeType {
                weight: 0,
                value: 0.0,
            },
        ];

        assert_eq!(max_bag_value(&cake_types, 4), 4.0);
    }
}
