pub use sys::rect::*;

#[cfg(test)]
mod test {
    use super::{Rect, Point};

    #[test]
    fn test_from_enclose_points() {
        assert_eq!(Rect::from_enclose_points(&[Point::new(2, 4), Point::new(5,9)], None),
                   Some(Rect::new(2, 4, 4, 6)));
        assert!(Rect::from_enclose_points(&[Point::new(0, 0), Point::new(10,10)],
                                          Some(Rect::new(3, 3, 1, 1))).is_none());
    }

    #[test]
    fn test_has_intersection() {
        assert!(Rect::new(0, 0, 10, 10).has_intersection(&Rect::new(9, 9, 10, 10)));
        // edge
        assert!(! Rect::new(0, 0, 10, 10).has_intersection(&Rect::new(10, 10, 10, 10)));
        // out
        assert!(! Rect::new(0, 0, 10, 10).has_intersection(&Rect::new(11, 11, 10, 10)));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(Rect::new(0, 0, 10, 10).intersection(&Rect::new(9, 9, 10, 10)),
                   Some(Rect::new(9, 9, 1, 1)));
        assert!(Rect::new(0, 0, 10, 10).intersection(&Rect::new(11, 11, 10, 10)).is_none());
    }

    #[test]
    fn test_union() {
        assert_eq!(Rect::new(0, 0, 1, 1).union(&Rect::new(9, 9, 1, 1)),
                   Rect::new(0, 0, 10, 10));
    }

    #[test]
    fn test_intersect_line() {
        assert_eq!(Rect::new(1, 1, 5, 5).intersect_line(&Point::new(0, 0), &Point::new(10, 10)),
                   Some((Point::new(1, 1), Point::new(5, 5))));
    }
 }
