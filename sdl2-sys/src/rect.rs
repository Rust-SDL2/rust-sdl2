/*!
Rectangle Functions
 */

#[cfg(feature = "no_std")]
use core::prelude::*;
#[cfg(feature = "no_std")]
use core::intrinsics::transmute;
#[cfg(not(feature = "no_std"))]
use std::mem::transmute;
use libc::c_int;

/// A structure that defines a two dimensional point.
#[derive(PartialEq, Clone, Debug, Copy)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

/// A structure that defines a rectangle, with the origin at the upper left.
#[derive(PartialEq, Clone, Debug, Copy)]
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
                transmute(clip.as_ref()),
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

