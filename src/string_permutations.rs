/*
Problem:
Given a input string, return a list of all possible permutations for that string.

Example:
Input: 'abc'
Output: ['abc', 'acb', 'bac', 'bca', 'cab', 'cba']
*/

pub fn string_permutation(input: &str) -> Vec<String> {
    recursive(input.chars().collect())
}

// Recursive solution, not optimized for time or space (as asked by interviewcake).
fn recursive(input: Vec<char>) -> Vec<String> {
    if input.len() == 1 {
        return vec![input[0].to_string()];
    }

    let mut permutations = vec![];
    for (index, &c) in input.iter().enumerate() {
        let mut inner_input = input.clone();
        inner_input.remove(index);
        for mut permutation in recursive(inner_input) {
            permutation.insert(0, c);
            permutations.push(permutation);
        }
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_basic() {
        assert_eq!(
            string_permutation("abc"),
            vec!["abc", "acb", "bac", "bca", "cab", "cba"]
        );
    }

    #[test]
    fn works_on_empty() {
        assert_eq!(string_permutation(""), Vec::<String>::new());
    }

    #[test]
    fn works_on_single() {
        assert_eq!(string_permutation("a"), vec!["a"]);
    }
}
