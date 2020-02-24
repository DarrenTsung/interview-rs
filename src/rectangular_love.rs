/*
Problem:
Given two Rectangles, return the intersection between the rectangles, if it exists.
*/
use std::cmp;

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
    min_x: i32,
    min_y: i32,

    width: u32,
    height: u32,
}

impl Rect {
    fn max_x(&self) -> i32 {
        self.min_x + (self.width as i32)
    }

    fn max_y(&self) -> i32 {
        self.min_y + (self.height as i32)
    }
}

pub fn intersection(a: &Rect, b: &Rect) -> Option<Rect> {
    let max_y = cmp::min(a.max_y(), b.max_y());
    let min_y = cmp::max(a.min_y, b.min_y);
    let max_x = cmp::min(a.max_x(), b.max_x());
    let min_x = cmp::max(a.min_x, b.min_x);

    // No intersection if no width / height.
    if max_y <= min_y || max_x <= min_x {
        return None;
    }

    Some(Rect {
        min_x,
        min_y,
        width: (max_x - min_x) as u32,
        height: (max_y - min_y) as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_intersection() {
        let a = Rect {
            min_x: 2,
            min_y: 1,
            width: 2,
            height: 2,
        };
        let b = Rect {
            min_x: 3,
            min_y: 0,
            width: 2,
            height: 2,
        };
        assert_eq!(
            intersection(&a, &b),
            Some(Rect {
                min_x: 3,
                min_y: 1,
                height: 1,
                width: 1
            })
        );
    }

    #[test]
    fn works_for_no_intersection() {
        let a = Rect {
            min_x: 0,
            min_y: 0,
            width: 1,
            height: 1,
        };
        let b = Rect {
            min_x: 2,
            min_y: 2,
            width: 2,
            height: 2,
        };
        assert_eq!(intersection(&a, &b), None);
    }

    #[test]
    fn works_for_no_intersection_edges_touching() {
        let a = Rect {
            min_x: 0,
            min_y: 0,
            width: 1,
            height: 1,
        };
        let b = Rect {
            min_x: 0,
            min_y: 1,
            width: 1,
            height: 1,
        };
        assert_eq!(intersection(&a, &b), None);
    }

    #[test]
    fn works_for_fully_enclosed() {
        let a = Rect {
            min_x: 0,
            min_y: 0,
            width: 2,
            height: 2,
        };
        let b = Rect {
            min_x: 1,
            min_y: 1,
            width: 1,
            height: 1,
        };
        assert_eq!(intersection(&a, &b), Some(b.clone()));
    }

    #[test]
    fn works_for_same_rect() {
        let a = Rect {
            min_x: 0,
            min_y: 0,
            width: 2,
            height: 2,
        };
        let b = a.clone();
        assert_eq!(intersection(&a, &b), Some(b.clone()));
    }
}
