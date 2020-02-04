use std::cmp;
use std::collections::HashSet;
use std::mem;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Meeting {
    start: i32,
    end: i32,
}

impl Meeting {
    pub fn new(start: i32, end: i32) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
}

pub fn merge_meetings(mut meetings: Vec<Meeting>) -> HashSet<Meeting> {
    meetings.sort_by_key(|m| m.start);

    let mut merged = HashSet::new();

    let mut meetings = meetings.into_iter();
    let mut current = if let Some(meeting) = meetings.next() {
        meeting
    } else {
        // Meetings is empty
        return merged;
    };

    for meeting in meetings {
        // If current does not overlap with meeting, then we know it
        // does not overlap with any other (since elements are sorted).
        if current.end < meeting.start {
            let current = mem::replace(&mut current, meeting);
            merged.insert(current);
            continue;
        }

        // If reached, current overlaps with meeting
        current.end = cmp::max(current.end, meeting.end);
    }
    merged.insert(current);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_meetings_works_base_case() {
        assert_eq!(
            merge_meetings(vec![
                Meeting::new(10, 12),
                Meeting::new(4, 8),
                Meeting::new(3, 5),
                Meeting::new(9, 10),
                Meeting::new(0, 1),
            ]),
            vec![Meeting::new(0, 1), Meeting::new(3, 8), Meeting::new(9, 12)]
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn merge_meetings_works_duplicates() {
        assert_eq!(
            merge_meetings(vec![
                Meeting::new(3, 5),
                Meeting::new(4, 8),
                Meeting::new(0, 1),
                Meeting::new(4, 8),
            ]),
            vec![Meeting::new(0, 1), Meeting::new(3, 8)]
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn merge_meetings_works_minimum_overlap() {
        assert_eq!(
            merge_meetings(vec![
                Meeting::new(3, 5),
                Meeting::new(5, 8),
                Meeting::new(0, 1),
            ]),
            vec![Meeting::new(0, 1), Meeting::new(3, 8)]
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn merge_meetings_works_multiple_overlaps() {
        assert_eq!(
            merge_meetings(vec![
                Meeting::new(3, 5),
                Meeting::new(9, 10),
                Meeting::new(5, 8),
                Meeting::new(7, 9),
                Meeting::new(0, 1),
            ]),
            vec![Meeting::new(0, 1), Meeting::new(3, 10)]
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn merge_meetings_works_complete_overlap() {
        assert_eq!(
            merge_meetings(vec![
                Meeting::new(3, 5),
                Meeting::new(4, 4),
                Meeting::new(0, 1),
            ]),
            vec![Meeting::new(0, 1), Meeting::new(3, 5)]
                .into_iter()
                .collect()
        )
    }
}
