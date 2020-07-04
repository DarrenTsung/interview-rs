// Given a string ("hello ellie") and a search string ("el"), return
// the HTML highlighted string: "h<b>el</b>lo <b>el</b>lie".
pub fn highlight(input: &str, search: &str) -> String {
    let mut highlighted = String::new();

    let mut in_progress_match = None;
    for i in 0..input.len() {
        let i_end = i + search.len();

        let can_compare_with_search = i_end <= input.len();
        if can_compare_with_search && &input[i..i_end] == search {
            in_progress_match = Some(if let Some((start, _end)) = in_progress_match {
                (start, i_end)
            } else {
                (i, i_end)
            });
        } else {
            match in_progress_match {
                None => highlighted.push_str(&input[i..=i]),
                Some((start, end)) => {
                    if i == end {
                        highlighted.push_str("<b>");
                        highlighted.push_str(&input[start..end]);
                        highlighted.push_str("</b>");
                        highlighted.push_str(&input[i..=i]);
                        in_progress_match = None;
                    }
                }
            }
        }
    }

    if let Some((start, end)) = in_progress_match {
        highlighted.push_str("<b>");
        highlighted.push_str(&input[start..end]);
        highlighted.push_str("</b>");
    }

    highlighted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(highlight("hello ellie", "el"), "h<b>el</b>lo <b>el</b>lie");
    }

    #[test]
    fn repeated() {
        assert_eq!(highlight("hoot", "o"), "h<b>oo</b>t");
        assert_eq!(highlight("oot", "o"), "<b>oo</b>t");
        assert_eq!(highlight("boo", "o"), "b<b>oo</b>");
    }

    #[test]
    fn overlapping() {
        assert_eq!(highlight("banana", "ana"), "b<b>anana</b>");
        assert_eq!(highlight("ananab", "ana"), "<b>anana</b>b");
        assert_eq!(highlight("anana", "ana"), "<b>anana</b>");
    }
}
