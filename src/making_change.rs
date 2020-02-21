/*
Problem:
Given an amount and coin denominations, determine the number of ways you
can give change to meet that amount.

Example:
Amount = 4, Denominations: [1, 2, 3]
Ways of making change = 4:
    1. [1, 1, 1, 1]
    2. [2, 1, 1]
    3. [2, 2]
    4. [3, 1]
*/
pub fn making_change(amount: u32, denominations: &[u32]) -> u32 {
    let recursive = recursive(amount, denominations);
    let bottom_up = bottom_up(amount, denominations);
    assert_eq!(bottom_up, recursive);
    recursive
}

fn bottom_up(amount: u32, denominations: &[u32]) -> u32 {
    let least_to_greatest_denominations = {
        let mut d = denominations.iter().cloned().collect::<Vec<_>>();
        d.sort();
        d
    };

    // This is a vec prefilled with `amount` -> 0
    let mut ways_to_make_change = {
        let mut ways = vec![];
        for _ in 0..=amount {
            ways.push(0);
        }
        ways
    };
    ways_to_make_change[0] = 1;

    for denomination in least_to_greatest_denominations {
        // Calculate ways to make change for current denominations based on previous denominations.
        for inner_amount in 1..=amount {
            if denomination > inner_amount {
                continue;
            }

            let prev_amount = inner_amount - denomination;
            ways_to_make_change[inner_amount as usize] += ways_to_make_change[prev_amount as usize];
        }
    }

    ways_to_make_change[amount as usize]
}

fn recursive(amount: u32, denominations: &[u32]) -> u32 {
    let greatest_to_least_denominations = {
        let mut d = denominations.iter().cloned().collect::<Vec<_>>();
        d.sort();
        d.reverse();
        d
    };

    recursive_helper(amount, &greatest_to_least_denominations)
}

fn recursive_helper(amount: u32, greatest_to_least_denominations: &[u32]) -> u32 {
    if amount == 0 {
        return 1;
    }

    let mut ways_to_make_change = 0;
    for index in 0..greatest_to_least_denominations.len() {
        let denomination = greatest_to_least_denominations[index];
        if denomination > amount {
            continue;
        }

        let amount = amount - denomination;
        ways_to_make_change += recursive_helper(amount, &greatest_to_least_denominations[index..]);
    }

    ways_to_make_change
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_for_example() {
        assert_eq!(making_change(4, &[1, 2, 3]), 4);
    }

    #[test]
    fn works_for_another_case() {
        assert_eq!(making_change(6, &[1, 2]), 4);
    }

    #[test]
    fn when_amount_is_zero() {
        assert_eq!(making_change(0, &[1, 2]), 1);
    }

    #[test]
    fn when_amount_can_not_be_made() {
        assert_eq!(making_change(4, &[3]), 0);
    }
}
