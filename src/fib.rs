// Computes the n-th fibonocci number.
pub fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut current = 1;

    let mut n = (n as i64) - 2;
    while n >= 0 {
        let new_current = prev + current;
        prev = current;
        current = new_current;
        n -= 1;
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);

        assert_eq!(fib(47), 2_971_215_073);
    }
}
