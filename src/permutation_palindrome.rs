use std::collections::HashMap;

pub fn has_permutation_palindrome(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut char_counts = HashMap::new();
    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    let mut has_char_with_odd_count = false;
    for (_c, count) in char_counts {
        if count % 2 == 0 {
            continue;
        }

        // Only one char with odd count is okay since the extra
        // character could be in the middle for a palindrome.
        if has_char_with_odd_count {
            return false;
        }

        has_char_with_odd_count = true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(has_permutation_palindrome("civic"), true);
    }

    #[test]
    fn works_with_permutation() {
        assert_eq!(has_permutation_palindrome("ivicc"), true);
    }

    #[test]
    fn false_cases_work() {
        assert_eq!(has_permutation_palindrome("civil"), false);
        assert_eq!(has_permutation_palindrome("livci"), false);
    }

    #[test]
    fn odd_character_counts_greater_than_1() {
        assert_eq!(has_permutation_palindrome("iii"), true);
        assert_eq!(has_permutation_palindrome("iiiii"), true);
        assert_eq!(has_permutation_palindrome("iiiiib"), false);
    }

    #[test]
    fn empty_string_returns_false() {
        assert_eq!(has_permutation_palindrome(""), false);
    }
}
