use std::collections::VecDeque;

pub fn merge_sorted_lists<T: std::fmt::Debug + PartialOrd>(
    mut a: VecDeque<T>,
    mut b: VecDeque<T>,
) -> Vec<T> {
    let mut sorted = vec![];

    loop {
        match (a.pop_front(), b.pop_front()) {
            (Some(item_a), Some(item_b)) => {
                if item_a > item_b {
                    sorted.push(item_b);
                    a.push_front(item_a);
                } else {
                    sorted.push(item_a);
                    b.push_front(item_b);
                }
            }
            (Some(a), None) => sorted.push(a),
            (None, Some(b)) => sorted.push(b),
            (None, None) => break,
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sorted_lists_works() {
        let my_list = vec![3, 4, 6, 10, 11, 15].into_iter().collect();
        let alices_list = vec![1, 5, 8, 12, 14, 19].into_iter().collect();

        assert_eq!(
            merge_sorted_lists(my_list, alices_list),
            vec![1, 3, 4, 5, 6, 8, 10, 11, 12, 14, 15, 19]
        );
    }
}
