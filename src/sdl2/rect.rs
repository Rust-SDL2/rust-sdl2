/*!
Rectangle Functions
 */

use std::mem;
use libc::c_int;

/// A structure that defines a two dimensional point.
#[deriving(PartialEq, Clone, Show)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

/// A structure that defines a rectangle, with the origin at the upper left.
#[deriving(PartialEq, Clone, Show)]
#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub mod ll {

    use libc::c_int;
    use super::Rect;
    use super::Point;

    pub type SDL_Rect = Rect;
    pub type SDL_Point = Point;
    pub type SDL_bool = c_int;

    extern "C" {
        pub fn SDL_HasIntersection(A: *const SDL_Rect, B: *const SDL_Rect) -> SDL_bool;
        pub fn SDL_IntersectRect(A: *const SDL_Rect, B: *const SDL_Rect, result: *const SDL_Rect) -> SDL_bool;
        pub fn SDL_UnionRect(A: *const SDL_Rect, B: *const SDL_Rect, result: *const SDL_Rect);
        pub fn SDL_EnclosePoints(points: *const SDL_Point, count: c_int, clip: *const SDL_Rect, result: *const SDL_Rect) -> SDL_bool;
        pub fn SDL_IntersectRectAndLine(rect: *const SDL_Rect, X1: *const c_int, Y1: *const c_int, X2: *const c_int, Y2: *const c_int) -> SDL_bool;
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    /// Calculate a minimal rectangle enclosing a set of points.
    pub fn from_enclose_points(points: &[Point], clip: Option<Rect>) -> Option<Rect> {
        let out: Rect = Rect::new(0, 0, 0, 0);

        let result = unsafe {
            ll::SDL_EnclosePoints(
                points.as_ptr(),
                points.len() as c_int,
                mem::transmute(clip.as_ref()),
                &out
            ) != 0
        };

        if result {
            Some(out)
        } else {
            None
        }
    }

    /// Check whether a rectangle has no area.
    pub fn is_empty(&self) -> bool {
        (self.w <= 0) || (self.h <= 0)
    }

    /// Determine whether two rectangles intersect.
    pub fn has_intersection(&self, other: &Rect) -> bool {
        unsafe {
            ll::SDL_HasIntersection(self, other) != 0
        }
    }

    /// Calculate the intersection of two rectangles.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let out: Rect = Rect::new(0, 0, 0, 0);

        let result = unsafe {
            ll::SDL_IntersectRect(self, other, &out) != 0
        };

        if result {
            Some(out)
        } else {
            None
        }
    }

    /// Calculate the union of two rectangles.
    pub fn union(&self, other: &Rect) -> Rect {
        let out: Rect = Rect::new(0, 0, 0, 0);

        unsafe {
            ll::SDL_UnionRect(self, other, &out)
        };

        out
    }

    /// Calculate the intersection of a rectangle and line segment. return points of intersection.
    pub fn intersect_line(&self, start: &Point, end: &Point) -> Option<(Point, Point)> {
        let out_start: Point = start.clone();
        let out_end: Point = end.clone();

        let result = unsafe {
            ll::SDL_IntersectRectAndLine(self, &out_start.x, &out_start.y, &out_end.x, &out_end.y) != 0
        };

        if result {
            Some((out_start, out_end))
        } else {
            None
        }
    }
}

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
