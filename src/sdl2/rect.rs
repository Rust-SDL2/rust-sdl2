//! Rectangles and points.
#![allow(const_err)]

use sys::rect as ll;
use std::mem;
use std::ptr;
use std::ops::{BitAnd, BitOr};

/// The maximal integer value that can be used for rectangles.
///
/// This value is smaller than strictly needed, but is useful in ensuring that
/// rect sizes will never have to be truncated when clamping.
pub fn max_int_value() -> u32 {
    i32::max_value() as u32 / 2
}

/// The minimal integer value that can be used for rectangle positions 
/// and points.
///
/// This value is needed, because otherwise the width of a rectangle created
/// from a point would be able to exceed the maximum width.
pub fn min_int_value() -> i32 {
    i32::min_value() / 2
}

fn clamp_size(val: u32) -> u32 {
    if val == 0 {
        1
    } else if val > max_int_value() {
        max_int_value()
    } else {
        val
    }
}

fn clamp_position(val: i32) -> i32 {
    if val > max_int_value() as i32 {
        max_int_value() as i32
    } else if val < min_int_value() {
        min_int_value()
    } else {
        val
    }
}

/// A rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    raw: ll::SDL_Rect,
}

impl Rect {
    /// Creates a new rectangle from the given values.
    ///
    /// The width and height are clamped to ensure that the right and bottom
    /// sides of the rectangle does not exceed i32::max_value().
    /// (The value 2147483647, the maximal positive size of an i32)
    ///
    /// This means that the rect size will behave oddly if you move it very far
    /// to the right or downwards on the screen.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        let raw = ll::SDL_Rect {
            x: clamp_position(x),
            y: clamp_position(y),
            w: clamp_size(width) as i32,
            h: clamp_size(height) as i32,
        };
        Rect { raw: raw }
    }
    
    /// Creates a new rectangle centered on the given position.
    ///
    /// The width and height are clamped to ensure that the right and bottom
    /// sides of the rectangle does not exceed i32::max_value().
    /// (The value 2147483647, the maximal positive size of an i32)
    ///
    /// This means that the rect size will behave oddly if you move it very far
    /// to the right or downwards on the screen.
    pub fn from_center<P>(center: P, width: u32, height: u32)
            -> Rect where P: Into<Point> {
        let raw = ll::SDL_Rect {
            x: 0,
            y: 0,
            w: clamp_size(width) as i32,
            h: clamp_size(height) as i32,
        };
        let mut rect = Rect { raw: raw };
        rect.center_on(center.into());
        rect
    }
    
    /// The horizontal position of this rectangle.
    pub fn x(&self) -> i32 {
        self.raw.x
    }
    
    /// The vertical position of this rectangle.
    pub fn y(&self) -> i32 {
        self.raw.y
    }
    
    /// The width of this rectangle.
    pub fn width(&self) -> u32 {
        self.raw.w as u32
    }
    
    /// The height of this rectangle.
    pub fn height(&self) -> u32 {
        self.raw.h as u32
    }
    
    /// Sets the horizontal position of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_x(&mut self, x: i32) {
        self.raw.x = clamp_position(x);
    }
    
    /// Sets the vertical position of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_y(&mut self, y: i32) {
        self.raw.y = clamp_position(y);
    }
    
    /// Sets the width of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_width(&mut self, width: u32) {
        self.raw.w = clamp_size(width) as i32;
    }
    
    /// Sets the height of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_height(&mut self, height: u32) {
        self.raw.h = clamp_size(height) as i32;
    }
    
    /// Returns the x-position of the left side of this rectangle.
    pub fn left(&self) -> i32 {
        self.raw.x
    }
    
    /// Returns the x-position of the right side of this rectangle.
    pub fn right(&self) -> i32 {
        self.raw.x + self.raw.w
    }
    
    /// Returns the y-position of the top side of this rectangle.
    pub fn top(&self) -> i32 {
        self.raw.y
    }
    
    /// Returns the y-position of the bottom side of this rectangle.
    pub fn bottom(&self) -> i32 {
        self.raw.y + self.raw.h
    }
    
    /// Returns the center of this rectangle.
    pub fn center(&self) -> Point {
        let x = self.raw.x + (self.raw.w / 2);
        let y = self.raw.y + (self.raw.h / 2);
        Point::new(x, y)
    }
    
    /// Sets the position of the right side of this rectangle to the given 
    /// value, clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_right(&mut self, right: i32) {
        self.raw.x = clamp_position(clamp_position(right) - self.raw.w);
    }
    
    /// Sets the position of the bottom side of this rectangle to the given 
    /// value, clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_bottom(&mut self, bottom: i32) {
        self.raw.y = clamp_position(clamp_position(bottom) - self.raw.h);
    }
    
    /// Centers the rectangle on the given point.
    pub fn center_on<P>(&mut self, point: P) where P: Into<(i32, i32)> {
        let (x, y) = point.into();
        self.raw.x = clamp_position(clamp_position(x) - self.raw.w / 2);
        self.raw.y = clamp_position(clamp_position(y) - self.raw. h / 2);
    }
    
    /// Move this rect and clamp the positions to prevent over/underflow.
    /// This also clamps the size to prevent overflow.
    pub fn offset(&mut self, x: i32, y: i32) {
        match self.raw.x.checked_add(x) {
            Some(val) => self.raw.x = clamp_position(val),
            None => {
                if x >= 0 {
                    self.raw.x = max_int_value() as i32;
                } else {
                    self.raw.x = i32::min_value();
                }
            },
        }
        match self.raw.y.checked_add(y) {
            Some(val) => self.raw.y = clamp_position(val),
            None => {
                if y >= 0 {
                    self.raw.y = max_int_value() as i32;
                } else {
                    self.raw.y = i32::min_value();
                }
            },
        }
    }
    
    /// Moves this rect to the given position after clamping the values.
    pub fn reposition<P>(&mut self, point: P) where P: Into<(i32, i32)> {
        let (x, y) = point.into();
        self.raw.x = clamp_position(x);
        self.raw.y = clamp_position(y);
    }
    
    /// Resizes this rect to the given size after clamping the values.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.raw.w = clamp_size(width) as i32;
        self.raw.h = clamp_size(height) as i32;
    }
    
    /// Checks whether this rect contains a given point.
    pub fn contains<P>(&self, point: P) -> bool where P: Into<(i32, i32)> {
        let (x, y) = point.into();
        let inside_x = x >= self.left() && x <= self.right();
        inside_x && (y >= self.top() && y <= self.bottom())
    }
    
    /// Returns the underlying C Rect.
    pub fn raw(&self) -> *const ll::SDL_Rect {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> *mut ll::SDL_Rect {
        self.raw() as *mut _
    }

    pub fn raw_slice(slice: &[Rect]) -> *const ll::SDL_Rect {
        unsafe { 
            mem::transmute(slice.as_ptr())
        }
    }

    pub fn from_ll(raw: ll::SDL_Rect) -> Rect {
        Rect::new(raw.x, raw.y, raw.w as u32, raw.h as u32)
    }
    
    /// Calculate a minimal rectangle enclosing a set of points.
    /// If a clipping rectangle is given, only points that are within it will be
    /// considered.
    pub fn from_enclose_points(points: &[Point], clipping_rect: Option<Rect>) 
            -> Option<Rect> {
        
        if points.len() == 0 {
            return None;
        }
        
        let mut out = unsafe {
            mem::uninitialized()
        };

        let clip_ptr = match clipping_rect.as_ref() {
            Some(r) => r.raw(),
            None => ptr::null()
        };

        let result = unsafe {
            ll::SDL_EnclosePoints(
                Point::raw_slice(points),
                points.len() as i32,
                clip_ptr,
                &mut out
            ) != 0
        };

        if result {
            // Return an error if the dimensions are too large.
            Some(Rect::from_ll(out))
        } else {
            None
        }
    }

    /// Determine whether two rectangles intersect.
    pub fn has_intersection(&self, other: Rect) -> bool {
        unsafe {
            ll::SDL_HasIntersection(self.raw(), other.raw()) != 0
        }
    }

    /// Calculate the intersection of two rectangles. 
    /// The bitwise AND operator `&` can also be used.
    pub fn intersection(&self, other: Rect) -> Option<Rect> {
        let mut out = unsafe { mem::uninitialized() };

        let success = unsafe {
            ll::SDL_IntersectRect(self.raw(), other.raw(), &mut out) != 0
        };

        if success {
            Some(Rect::from_ll(out))
        } else {
            None
        }
    }

    /// Calculate the union of two rectangles. 
    /// The bitwise OR operator `|` can also be used.
    pub fn union(&self, other: Rect) -> Rect {
        let mut out = unsafe {
            mem::uninitialized()
        };

        unsafe {
            // If `self` and `other` are both empty, `out` remains uninitialized.
            // Because empty rectangles aren't allowed in Rect, we don't need to worry about this.
            ll::SDL_UnionRect(self.raw(), other.raw(), &mut out)
        };

        Rect::from_ll(out)
    }

    /// Calculates the intersection of a rectangle and a line segment and 
    /// returns the points of their intersection.
    pub fn intersect_line(&self, start: Point, end: Point)
            -> Option<(Point, Point)> {
        
        let (mut start_x, mut start_y) = (start.x(), start.y());
        let (mut end_x, mut end_y) = (end.x(), end.y());

        let intersected = unsafe {
            ll::SDL_IntersectRectAndLine(
                self.raw(), 
                &mut start_x, &mut start_y,
                &mut end_x, &mut end_y
            ) != 0
        };

        if intersected {
            Some((Point::new(start_x, start_y), Point::new(end_x, end_y)))
        } else {
            None
        }
    }
}

impl Into<(i32, i32, u32, u32)> for Rect {
    fn into(self) -> (i32, i32, u32, u32) {
        (self.raw.x, self.raw.y, self.raw.w as u32, self.raw.h as u32)
    }
}

impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, width, height): (i32, i32, u32, u32)) -> Rect {
        Rect::new(x, y, width, height)
    }
}

// Intersection
impl BitAnd<Rect> for Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Rect) -> Option<Rect> { self.intersection(rhs) }
}

// Union
impl BitOr<Rect> for Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect { self.union(rhs) }
}

/// Immutable point type, consisting of x and y.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Point {
    raw: ll::SDL_Point
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point::new(x, y)
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x(), self.y())
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            raw: ll::SDL_Point { 
                x: clamp_position(x),
                y: clamp_position(y),
            }
        }
    }
    
    pub fn from_ll(raw: ll::SDL_Point) -> Point {
        Point::new(raw.x, raw.y)
    }
    
    pub fn raw_slice(slice: &[Point]) -> *const ll::SDL_Point {
        unsafe {
            mem::transmute(slice.as_ptr())
        }
    }
    
    pub fn raw(&self) -> *const ll::SDL_Point { 
        &self.raw
    }

    pub fn offset(&self, x: i32, y: i32) -> Point {
        let x = match self.raw.x.checked_add(x) {
            Some(val) => val,
            None => {
                if x < 0 {
                    min_int_value()
                } else {
                    max_int_value() as i32
                }
            }
        };
        let y = match self.raw.y.checked_add(y) {
            Some(val) => val,
            None => {
                if y < 0 {
                    min_int_value()
                } else {
                    max_int_value() as i32
                }
            }
        };
        Point::new(x, y)
    }

    pub fn x(&self) -> i32 {
        self.raw.x
    }
    
    pub fn y(&self) -> i32 {
        self.raw.y
    }
}

#[cfg(test)]
mod test {
    use super::{Rect, Point, max_int_value, min_int_value};
    
    /// Used to compare "literal" (unclamped) rect values.
    fn tuple(x: i32, y: i32, w: u32, h: u32) -> (i32, i32, u32, u32) {
        (x, y, w, h)
    }

    #[test]
    fn enclose_points_valid() {
        assert_eq!(
            Some(tuple(2, 4, 4, 6)),
            Rect::from_enclose_points(
                &[Point::new(2, 4), Point::new(5,9)], 
                None
            ).map(|r| r.into())
        );
    }
    
    #[test]
    fn enclose_points_outside_clip_rect() {
        assert_eq!(
            Rect::from_enclose_points(
                &[Point::new(0, 0), Point::new(10,10)], 
                Some(Rect::new(3, 3, 1, 1))), 
            None
        );
    }
    
    #[test]
    fn enclose_points_max_values() {
        // Try to enclose the top-left-most and bottom-right-most points.
        assert_eq!(
            Some(tuple(
                min_int_value(), min_int_value(), 
                max_int_value(), max_int_value()
            )),
            Rect::from_enclose_points(
                &[Point::new(i32::min_value(), i32::min_value()), 
                Point::new(i32::max_value(), i32::max_value())], None
            ).map(|r| r.into())
        );
    }

    #[test]
    fn has_intersection() {
        let rect = Rect::new(0, 0, 10, 10);
        assert!(rect.has_intersection(Rect::new(9, 9, 10, 10)));
        // edge
        assert!(! rect.has_intersection(Rect::new(10, 10, 10, 10)));
        // out
        assert!(! rect.has_intersection(Rect::new(11, 11, 10, 10)));
    }

    #[test]
    fn intersection() {
        let rect = Rect::new(0, 0, 10, 10);
        assert_eq!(
            rect & Rect::new(9, 9, 10, 10),
            Some(Rect::new(9, 9, 1, 1))
        );
        assert_eq!(
            rect & Rect::new(11, 11, 10, 10),
            None
        );
    }

    #[test]
    fn union() {
        assert_eq!(
            Rect::new(0, 0, 1, 1) | Rect::new(9, 9, 1, 1), 
            Rect::new(0, 0, 10, 10)
        );
    }

    #[test]
    fn intersect_line() {
        assert_eq!(
            Rect::new(1, 1, 5, 5).intersect_line(
                Point::new(0, 0), Point::new(10, 10)
            ),
            Some((Point::new(1, 1), Point::new(5, 5)))
        );
    }
    
    #[test]
    fn clamp_size_zero() {
        assert_eq!(
            tuple(0, 0, 1, 1),
            Rect::new(0, 0, 0, 0).into()
        );
    }
    
    #[test]
    fn clamp_position_min() {
        assert_eq!(
            tuple(min_int_value(), min_int_value(), 1, 1),
            Rect::new(i32::min_value(), i32::min_value(), 1, 1).into()
        );
    }
    
    #[test]
    fn clamp_size_max() {
        assert_eq!(
            tuple(0, 0, max_int_value(), max_int_value()),
            Rect::new(0, 0, max_int_value() + 1, max_int_value() + 1).into()
        );
    }
    
    #[test]
    fn clamp_i32_max() {
        assert_eq!(
            tuple(0, 0, max_int_value(), max_int_value()),
            Rect::new(
                0, 0, i32::max_value() as u32, i32::max_value() as u32
            ).into()
        )
    }
    
    #[test]
    fn clamp_position_max() {
        assert_eq!(
            tuple(max_int_value() as i32, max_int_value() as i32, 1, 1),
            Rect::new(
                max_int_value() as i32 + 1, max_int_value() as i32 + 1, 1, 1
            ).into()
        );
    }
    
    #[test]
    fn rect_into() {
        let test: (i32, i32, u32, u32) = (-11, 5, 50, 20);
        assert_eq!(
            test,
            Rect::new(-11, 5, 50, 20).into()
        );
    }
    
    #[test]
    fn rect_from() {
        assert_eq!(
            Rect::from((-11, 5, 50, 20)), 
            Rect::new(-11, 5, 50, 20)
        );
    }
    
    #[test]
    fn point_into() {
        let test: (i32, i32) = (-11, 5);
        assert_eq!(
            test,
            Point::new(-11, 5).into()
        );
    }
    
    #[test]
    fn point_from() {
        let test: (i32, i32) = (-11, 5);
        assert_eq!(
            test,
            Point::new(-11, 5).into()
        );
    }
 }