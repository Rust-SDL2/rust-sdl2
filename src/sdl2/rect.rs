//! Rectangles and points.

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
    } else {
        val
    }
}

/// A rectangle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn reposition(&mut self, x: i32, y: i32) {
        self.raw.x = clamp_position(x);
        self.raw.y = clamp_position(y);
    }
    
    /// Resizes this rect to the given size after clamping the values.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.raw.w = clamp_size(width) as i32;
        self.raw.h = clamp_size(height) as i32;
    }
    
    /// Returns the underlying C Rect.
    pub fn raw(&self) -> *const ll::SDL_Rect {
        &self.raw
    }
    
    pub fn raw_from_option(rect: Option<&Rect>) -> *const ll::SDL_Rect {
        match rect {
            Some(ref r) => r.raw(),
            None => ptr::null()
        }
    }

    pub fn raw_mut_from_option(v: Option<&mut Rect>) -> *mut ll::SDL_Rect {
        match v {
            Some(ref r) => r.raw() as *mut _,
            None => ptr::null_mut()
        }
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
    pub fn has_intersection(&self, other: &Rect) -> bool {
        unsafe {
            ll::SDL_HasIntersection(self.raw(), other.raw()) != 0
        }
    }

    /// Calculate the intersection of two rectangles. 
    /// The bitwise AND operator `&` can also be used.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
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
    pub fn union(&self, other: &Rect) -> Rect {
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
    fn bitand(self, rhs: Rect) -> Option<Rect> { self.intersection(&rhs) }
}

impl<'a> BitAnd<&'a Rect> for Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: &Rect) -> Option<Rect> { self.intersection(rhs) }
}

impl<'a> BitAnd<Rect> for &'a Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Rect) -> Option<Rect> { self.intersection(&rhs) }
}

impl<'a> BitAnd<&'a Rect> for &'a Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: &Rect) -> Option<Rect> { self.intersection(rhs) }
}

// Union
impl BitOr<Rect> for Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect { self.union(&rhs) }
}

impl<'a> BitOr<&'a Rect> for Rect {
    type Output = Rect;
    fn bitor(self, rhs: &Rect) -> Rect { self.union(rhs) }
}

impl<'a> BitOr<Rect> for &'a Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect { self.union(&rhs) }
}

impl<'a> BitOr<&'a Rect> for &'a Rect {
    type Output = Rect;
    fn bitor(self, rhs: &Rect) -> Rect { self.union(rhs) }
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
            raw: ll::SDL_Point { x: x, y: y }
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
        Point::new(self.x() + x, self.y() + y)
    }

    pub fn x(&self) -> i32 {
        self.raw.x
    }
    
    pub fn y(&self) -> i32 {
        self.raw.y
    }
}

/* // Deprecated at the moment. Sorry.
#[cfg(test)]
mod test {
    use super::{Rect, Point};

    fn rect(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Rect::new(x, y, w, h).unwrap()
    }

    #[test]
    fn test_from_enclose_points() {
        use std::i32;

        assert_eq!(
            Rect::from_enclose_points(
                &[Point::new(2, 4), Point::new(5,9)], 
                None
            ),
            Ok(rect(2, 4, 4, 6))
        );
        assert_eq!(
            Rect::from_enclose_points(
                &[Point::new(0, 0), Point::new(10,10)], 
                Some(rect(3, 3, 1, 1))), Ok(None));

        // Try to enclose the top-left-most and bottom-right-most points.
        // The rectangle will be too large, and the function should return an error.
        assert!(Rect::from_enclose_points(&[Point::new(i32::MIN, i32::MIN), Point::new(i32::MAX, i32::MAX)], None).is_err());
    }

    #[test]
    fn test_has_intersection() {
        assert!(rect(0, 0, 10, 10).has_intersection(&rect(9, 9, 10, 10)));
        // edge
        assert!(! rect(0, 0, 10, 10).has_intersection(&rect(10, 10, 10, 10)));
        // out
        assert!(! rect(0, 0, 10, 10).has_intersection(&rect(11, 11, 10, 10)));
    }

    #[test]
    fn test_intersection() {
        assert_eq!(rect(0, 0, 10, 10) & rect(9, 9, 10, 10), Some(rect(9, 9, 1, 1)));
        assert!((rect(0, 0, 10, 10) & rect(11, 11, 10, 10)).is_none());
    }

    #[test]
    fn test_union() {
        assert_eq!(rect(0, 0, 1, 1) | rect(9, 9, 1, 1), rect(0, 0, 10, 10));
    }

    #[test]
    fn test_intersect_line() {
        assert_eq!(rect(1, 1, 5, 5).intersect_line(Point::new(0, 0), Point::new(10, 10)),
                   Some((Point::new(1, 1), Point::new(5, 5))));
    }

    #[test]
    fn test_rect_invariants() {
        use std::i32;
        const MAX_SIZE: u32 = i32::MAX as u32;

        let pass_nonempty = &[(0, 0, 1, 1), (i32::MIN, i32::MIN, MAX_SIZE, MAX_SIZE), (0, 0, MAX_SIZE, MAX_SIZE)];
        let pass_empty = &[(0, 0, 1, 0), (i32::MAX, i32::MAX, 0, 0)];
        let fail = &[(1, 1, MAX_SIZE, MAX_SIZE), (-1, -1, MAX_SIZE+1, MAX_SIZE+1), (i32::MAX, i32::MAX, 1, 1)];

        for &(x, y, w, h) in pass_nonempty {
            // Should be non-empty.
            match Rect::new(x, y, w, h) {
                Ok(None) | Err(..) => panic!("{:?}", (x, y, w, h)),
                _ => ()
            }
        }

        for &(x, y, w, h) in pass_empty {
            // Should be empty.
            match Rect::new(x, y, w, h) {
                Ok(Some(..)) | Err(..) => panic!("{:?}", (x, y, w, h)),
                _ => ()
            }
        }

        for &(x, y, w, h) in fail {
            // Should fail.
            assert!(Rect::new(x, y, w, h).is_err(), "{:?}", (x, y, w, h));
        }
    }

    #[test]
    fn test_rect_convert() {
        // Rect into
        let r = rect(-11, 5, 50, 20);
        let r_tuple: (i32, i32, u32, u32) = r.into();
        assert_eq!(r.xywh(), r_tuple);

        // Point into
        let p = Point::new(-11, 5);
        let p_tuple = p.into();
        assert_eq!(p.xy(), p_tuple);

        // Point from
        let p_tuple = (-11, 5);
        let p: Point = Point::from(p_tuple);
        assert_eq!(p, Point::new(-11, 5));
    }
 }
 */