use std::collections::HashSet;

pub fn has_two_movies_for_flight(flight_length: u32, movie_lengths: Vec<u32>) -> bool {
    let mut complement_movie_lengths = HashSet::new();

    for movie_length in movie_lengths {
        // If movie is not valid, ignore.
        if movie_length > flight_length {
            continue;
        }

        // If movie_length exists in this set, then that means that
        // a movie exists with the complement movie length.
        if complement_movie_lengths.contains(&movie_length) {
            return true;
        }

        complement_movie_lengths.insert(flight_length - movie_length);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(has_two_movies_for_flight(10, vec![8, 1, 1, 2, 7]), true);
    }

    #[test]
    fn movie_longer_than_flight_works() {
        assert_eq!(has_two_movies_for_flight(10, vec![8, 1, 2000, 7, 2]), true);
    }

    #[test]
    fn works_with_movie_length_zero() {
        assert_eq!(has_two_movies_for_flight(10, vec![0, 1, 7, 10]), true);
    }

    #[test]
    fn no_pair_with_half_length() {
        assert_eq!(has_two_movies_for_flight(10, vec![8, 5, 1, 1, 4, 7]), false);
    }

    #[test]
    fn pair_with_half_length() {
        assert_eq!(
            has_two_movies_for_flight(10, vec![8, 5, 1, 1, 4, 5, 7]),
            true
        );
    }
}
