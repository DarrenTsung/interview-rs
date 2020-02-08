use std::cmp;

pub fn highest_product_of_three(integers: Vec<i32>) -> i64 {
    if integers.len() < 3 {
        return 0;
    }

    // An array holding lowest numbers, carries 2 elements by the end.
    let mut lowest = vec![];
    // An array holding highest numbers, carries 3 elements by the end.
    let mut highest = vec![];

    for i in integers {
        let i = i as i64;

        if lowest.len() < 2 {
            lowest.push(i);
        } else {
            let max_of_lowest = *lowest.iter().max().expect("exists");
            if i < max_of_lowest {
                let index = lowest
                    .iter()
                    .position(|&x| max_of_lowest == x)
                    .expect("exists");
                lowest[index] = i;
            }
        }

        if highest.len() < 3 {
            highest.push(i);
        } else {
            let min_of_highest = *highest.iter().min().expect("exists");
            if i > min_of_highest {
                let index = highest
                    .iter()
                    .position(|&x| min_of_highest == x)
                    .expect("exists");
                highest[index] = i;
            }
        }
    }

    let highest_integer = *highest.iter().max().expect("exists");
    cmp::max(
        highest.iter().product(),
        lowest.iter().product::<i64>() * highest_integer,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_positive() {
        assert_eq!(highest_product_of_three(vec![1, 3, 4, 2]), 24);
    }

    #[test]
    fn one_negative() {
        assert_eq!(highest_product_of_three(vec![-1, 2, 3, 4]), 24);
        assert_eq!(highest_product_of_three(vec![-1, 3, 4]), -12);
        assert_eq!(highest_product_of_three(vec![-4, 3, 1, 4]), 12);
    }

    #[test]
    fn two_negative_highest() {
        assert_eq!(highest_product_of_three(vec![1, 2, -2, -3, 4]), 24);
    }

    #[test]
    fn all_negative() {
        assert_eq!(highest_product_of_three(vec![-1, -2, -3, -4]), -6);
    }
}
