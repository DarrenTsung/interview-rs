/// Given a message like "help me", it should mutate message to: "me help".
pub fn reverse_words_order(message: &mut [char]) {
    // Find alphabetic boundaries.
    let mut first_alpha = 0;
    let mut last_alpha = message.len() - 1;

    while first_alpha <= last_alpha {
        if message[first_alpha].is_alphabetic() {
            break;
        }
        first_alpha += 1;
    }

    while last_alpha >= first_alpha {
        if message[last_alpha].is_alphabetic() {
            break;
        }
        last_alpha -= 1;
    }

    message[first_alpha..=last_alpha].reverse();
    reverse_words(&mut message[first_alpha..=last_alpha]);
}

/// Reverses each word in a message. For example, given a message like:
/// "pleh em", it should mutate to: "help me".
fn reverse_words(message: &mut [char]) {
    let mut current_head = 0;
    let message_len = message.len();
    for index in 0..message_len {
        if message[index].is_alphabetic() {
            continue;
        }

        // Found word, reverse.
        if index > 0 && current_head < index - 1 {
            reverse(&mut message[current_head..index])
        }

        // Assume next character is alphabetic, current_head will
        // be replaced if not so anyways.
        current_head = index + 1;
    }

    // Handle ending word.
    if current_head < message_len - 1 {
        reverse(&mut message[current_head..message_len])
    }
}

fn reverse<T>(slice: &mut [T]) {
    let mut front = 0;
    let mut back = slice.len() - 1;

    while front < back {
        slice.swap(front, back);
        front += 1;
        back -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper for taking in strings during tests.
    fn reverse_words_order_in_string(s: &mut String) {
        let mut chars = s.chars().collect::<Vec<_>>();
        reverse_words_order(&mut chars);
        *s = chars.into_iter().collect();
    }

    #[test]
    fn reverse_words_order_works() {
        let mut input = "cake pound steal".into();
        let output = "steal pound cake";

        reverse_words_order_in_string(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn reverse_words_order_works_ends_with_non_alphabetic() {
        let mut input = "foo bar ".into();
        let output = "bar foo ";

        reverse_words_order_in_string(&mut input);
        assert_eq!(input, output);
    }
}
