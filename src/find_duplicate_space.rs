// Find the duplicate given the following constraints:
// 1. The integers are in the range 1..n
// 2. The list has a length of n+1
pub fn find_duplicate(values: Vec<i32>) -> i32 {
    let n = values.len() - 1;

    let mut lower = 1;
    let mut upper = n as i32;

    loop {
        let mid = (lower + upper) / 2;
        let mut in_lower = 0;
        let mut in_upper = 0;
        let mut in_mid = 0;
        for &value in &values {
            if value < mid {
                in_lower += 1;
            } else if value > mid {
                in_upper += 1;
            } else {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(find_duplicate(vec![1, 3, 3, 2]), 3);
        assert_eq!(find_duplicate(vec![1, 2, 3, 2]), 2);
        assert_eq!(find_duplicate(vec![2, 3, 1, 1]), 1);
    }
}
