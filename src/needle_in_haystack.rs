/*
Problem:
Given a haystack to search (string) like "aaabcdddbbddddabcdefghi" and a needle
like "abc", returns the indicies in haystack where needle can be found.

For this example, the return result would be: [2, 14].
*/
pub fn needle_in_haystack(haystack: &str, needle: &str) -> Vec<usize> {
    let naive = needle_in_haystack_naive(haystack, needle);
    let rk = needle_in_haystack_rk(haystack, needle);
    assert_eq!(naive, rk);
    naive
}

/*
Naive Solution:
For each index in the haystack (up until the remaining slice is smaller
than the needle), check if slice matches the needle.

Time complexity:
O(h*n) where h is # of chars in haystack and n is # of chars in needle.
This is since we have to potentially check n characters for each index in haystack.

Space complexity:
O(1) since we're just keeping track of indicies.
*/
fn needle_in_haystack_naive(haystack: &str, needle: &str) -> Vec<usize> {
    if needle.is_empty() {
        return vec![];
    }

    let mut found_indices = vec![];

    for i in 0..haystack.len() {
        // If end of slice is out-of-bounds, exit.
        let slice_end = i + needle.len();
        if slice_end > haystack.len() {
            break;
        }

        if &haystack[i..slice_end] == needle {
            found_indices.push(i);
        }
    }

    found_indices
}

/*
More efficient solution (Rabin-Karp Algorithm (RK)):
To get to a more efficient time complexity, we can implement RK and calculate
a hash for the string slice. The hash for the next slice (shifted chars) can be
calculated in O(1) time.

Time complexity:
O(h) where h is # of chars in haystack.

Space complexity:
O(1) since we just storing hash.
*/
fn needle_in_haystack_rk(haystack: &str, needle: &str) -> Vec<usize> {
    if needle.is_empty() {
        return vec![];
    }

    let mut found_indices = vec![];
    let haystack_chars = haystack.chars().collect::<Vec<_>>();

    // Use a large prime smaller than than u32::MAX / BASE (256).
    const PRIME: u32 = 104_173;

    // max_base is equal to the base (256) to the needle.len() - 1 power.
    // This is used to 'pop' the char being removed from the hash.
    let max_base = {
        let mut e = 256;
        for _ in 2..needle.len() {
            e = (e * e) % PRIME;
        }
        e
    };

    let needle_hash = calculate_hash(needle, PRIME);
    let mut slice_hash = None;
    for i in 0..haystack.len() {
        // If end of slice is out-of-bounds, exit.
        let slice_end = i + needle.len();
        if slice_end > haystack.len() {
            break;
        }

        let current_hash = if let Some(mut hash) = slice_hash {
            // Calculate new hash with new character.
            let to_remove = (haystack_chars[i - 1] as u32 * max_base) % PRIME;
            if to_remove > hash {
                hash = PRIME - (to_remove - hash);
            } else {
                hash = (hash - to_remove) % PRIME;
            }
            hash = (hash * 256) % PRIME;
            hash = (hash + haystack_chars[slice_end - 1] as u32) % PRIME;
            hash
        } else {
            calculate_hash(&haystack[..slice_end], PRIME)
        };

        if current_hash == needle_hash {
            // Need to check that collision wasn't false positive.
            if &haystack[i..slice_end] == needle {
                found_indices.push(i);
            }
        }

        slice_hash = Some(current_hash);
    }

    found_indices
}

fn calculate_hash(slice: &str, modulo: u32) -> u32 {
    let mut hash = 0u32;
    for c in slice.chars() {
        let ascii_code = c as u8;
        hash = (hash * 256 + ascii_code as u32) % modulo;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_example() {
        assert_eq!(
            needle_in_haystack("aaabcdddbbddddabcdefghi", "abc"),
            vec![2, 14]
        );
    }

    #[test]
    fn all_matching() {
        assert_eq!(needle_in_haystack("aaaa", "aa"), vec![0, 1, 2]);
    }

    #[test]
    fn haystack_smaller_than_hash() {
        assert_eq!(needle_in_haystack("aa", "aaa"), vec![]);
    }

    #[test]
    fn no_matches() {
        assert_eq!(needle_in_haystack("abcd", "aa"), vec![]);
    }

    #[test]
    fn needle_empty() {
        assert_eq!(needle_in_haystack("aaaa", ""), vec![]);
    }
}
