use std::cmp;

pub fn get_max_profit(stock_prices: Vec<u32>) -> i64 {
    if stock_prices.is_empty() {
        return 0;
    }

    let mut max_profit = None;
    let mut lowest = stock_prices[0];
    for price in stock_prices.into_iter().skip(1) {
        let profit = price as i64 - lowest as i64;

        lowest = cmp::min(lowest, price);

        if let Some(prev_max_profit) = max_profit {
            max_profit = Some(cmp::max(prev_max_profit, profit));
        } else {
            max_profit = Some(profit);
        }
    }

    max_profit.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(get_max_profit(vec![10, 7, 5, 8, 11, 9]), 6);
    }

    #[test]
    fn negative_profit() {
        assert_eq!(get_max_profit(vec![13, 11, 8, 6, 1, 0]), -1);
    }

    #[test]
    fn empty() {
        assert_eq!(get_max_profit(vec![]), 0);
    }

    #[test]
    fn only_one() {
        assert_eq!(get_max_profit(vec![5]), 0);
    }
}
