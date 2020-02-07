pub fn validate_fifo_orders(take_out: Vec<i32>, dine_in: Vec<i32>, served: Vec<i32>) -> bool {
    let mut take_out_index = 0;
    let mut dine_in_index = 0;

    for id in served {
        // If this id is not found in the pointers for take_out / dine_in,
        // then the customers were not served in fifo order.
        let was_take_out = take_out_index < take_out.len() && id == take_out[take_out_index];
        let was_dine_in = dine_in_index < dine_in.len() && id == dine_in[dine_in_index];

        if !was_take_out && !was_dine_in {
            return false;
        }

        if was_take_out {
            take_out_index += 1;
        }
        if was_dine_in {
            dine_in_index += 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_fifo() {
        let is_fifo = validate_fifo_orders(vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 4, 6, 5, 3]);
        assert_eq!(is_fifo, false);
    }

    #[test]
    fn fifo() {
        let is_fifo = validate_fifo_orders(vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 3, 5, 4, 6]);
        assert_eq!(is_fifo, true);
    }

    #[test]
    fn empty() {
        let is_fifo = validate_fifo_orders(vec![], vec![2, 4, 6], vec![2, 4, 6]);
        assert_eq!(is_fifo, true);
    }

    #[test]
    fn repeats() {
        let is_fifo = validate_fifo_orders(vec![10], vec![2, 4, 6, 4], vec![2, 10, 4, 6, 4]);
        assert_eq!(is_fifo, true);
    }
}
