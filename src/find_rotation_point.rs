pub fn find_rotation_point(list: &[&str]) -> usize {
    let mut lower_index = 0;
    let mut upper_index = list.len() - 1;

    while lower_index < upper_index - 1 {
        let upper = list[upper_index];

        let mid_index = (lower_index + upper_index) / 2;
        let mid = list[mid_index];

        if mid > upper {
            lower_index = mid_index;
        } else {
            upper_index = mid_index;
        }
    }

    upper_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let words = vec!["p", "r", "s", "u", "x", "a", "b", "c", "e", "k", "o"];
        assert_eq!(find_rotation_point(&words), 5);

        let words = vec![
            "p", "r", "s", "u", "x", "x", "x", "x", "y", "a", "b", "c", "e", "k", "o",
        ];
        assert_eq!(find_rotation_point(&words), 9);

        let words = vec!["p", "s", "b", "o"];
        assert_eq!(find_rotation_point(&words), 2);
    }
}
