pub fn products_of_all_except_at_index(values: Vec<i32>) -> Vec<i32> {
    if values.len() < 2 {
        return vec![];
    }

    let mut products = vec![];
    for _ in &values {
        products.push(1);
    }

    let mut product_before_index = 1;
    for (index, &value) in values.iter().enumerate() {
        products[index] *= product_before_index;
        product_before_index *= value;
    }

    let mut product_after_index = 1;
    for (index, &value) in values.iter().enumerate().rev() {
        products[index] *= product_after_index;
        product_after_index *= value;
    }

    products
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(
            products_of_all_except_at_index(vec![1, 7, 3, 4]),
            vec![84, 12, 28, 21]
        );
    }

    #[test]
    fn has_zeros() {
        assert_eq!(
            products_of_all_except_at_index(vec![1, 0, 3, 4]),
            vec![0, 12, 0, 0]
        );
    }

    #[test]
    fn negatives() {
        assert_eq!(
            products_of_all_except_at_index(vec![1, -1, 3, 4]),
            vec![-12, 12, -4, -3]
        );
    }

    #[test]
    fn less_than_2_elems() {
        assert_eq!(products_of_all_except_at_index(vec![3]), vec![]);
        assert_eq!(products_of_all_except_at_index(vec![]), vec![]);
    }
}
