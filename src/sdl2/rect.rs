use sys::rect as ll;
use std::mem;
use std::ptr;
use std::ops::{BitAnd, BitOr};
use util::validate_int;
use get_error;

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
        self.xy()
    }
}

impl Point {
    #[inline]
    pub fn raw(&self) -> *const ll::SDL_Point { &self.raw }

    #[inline]
    pub fn raw_slice(slice: &[Point]) -> *const ll::SDL_Point {
        unsafe { mem::transmute(slice.as_ptr()) }
    }

    #[inline]
    pub fn from_ll(raw: ll::SDL_Point) -> Point {
        Point::new(raw.x, raw.y)
    }

    #[inline]
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            raw: ll::SDL_Point { x: x, y: y }
        }
    }

    #[inline]
    pub fn offset(&self, x: i32, y: i32) -> Point {
        Point::new(self.x() + x, self.y() + y)
    }

    #[inline]
    pub fn xy(&self) -> (i32, i32) {
        (self.raw.x, self.raw.y)
    }

    #[inline] pub fn x(&self) -> i32 { self.raw.x }
    #[inline] pub fn y(&self) -> i32 { self.raw.y }
}

/// Immutable rectangle type, consisting of x, y, width and height.
///
/// Rectangle invariants:
///
/// * `width` and `height` are positive and non-zero.
/// * `width` and `height` are less than `1<<31` (2,147,483,648).
/// * `x + width` and `y + height` do not overflow.
///
/// These invariants exist in the wrapper because many SDL functions that accept rectangles don't
/// behave predictably if the above conditions aren't met.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Rect {
    raw: ll::SDL_Rect
}

// We can't implement From, so it's just Into for Rect.

impl Into<(i32, i32, u32, u32)> for Rect {
    fn into(self) -> (i32, i32, u32, u32) {
        self.xywh()
    }
}

impl Rect {
    #[inline]
    pub fn raw(&self) -> *const ll::SDL_Rect { &self.raw }

    #[inline]
    pub fn raw_from_option(v: Option<&Rect>) -> *const ll::SDL_Rect {
        match v {
            Some(ref r) => r.raw(),
            None => ptr::null()
        }
    }

    #[inline]
    pub fn raw_mut_from_option(v: Option<&mut Rect>) -> *mut ll::SDL_Rect {
        match v {
            Some(ref r) => r.raw() as *mut _,
            None => ptr::null_mut()
        }
    }

    #[inline]
    pub fn raw_slice(slice: &[Rect]) -> *const ll::SDL_Rect {
        unsafe { mem::transmute(slice.as_ptr()) }
    }

    #[inline]
    pub fn from_ll(raw: ll::SDL_Rect) -> Result<Rect, String> {
        if raw.w == 0 {
            Err("The width of the C rect is zero".to_owned())
        } else if raw.h == 0 {
            Err("The height of the C rect is zero".to_owned())
        } else {
            Rect::new(raw.x, raw.y, raw.w as u32, raw.h as u32)
        }
    }

    /// Creates a new rectangle.
    ///
    /// If `width` or `height` is zero, `Ok(None)` is returned.
    /// If the arguments violate any of the other rectangle invariants, an error is returned.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Result<Rect, String> {
        let width = try!(validate_int(width));
        let height = try!(validate_int(height));

        if x.checked_add(width).is_none() {
            Err("`x` + `width` overflows.".to_owned())
        } else if y.checked_add(height).is_none() {
            Err("`y` + `height` overflows.".to_owned())
        } else {
            match (width, height) {
                (0, _) => {
                    Err("The width is zero.".to_owned())
                },
                (_, 0) => {
                    Err("The height is zero.".to_owned())
                },
                (w, h) => {
                    Ok( Rect {
                        raw: ll::SDL_Rect {
                            x: x,
                            y: y,
                            w: w,
                            h: h
                        }
                    })
                }
            }
        }
    }

    /// Offsets the rectangle's x and y coordinates.
    ///
    /// If the new rectangle violates any invariants, an error is returned.
    #[inline]
    pub fn offset(&self, x: i32, y: i32) -> Result<Rect, String> {
        Rect::new(self.x() + x, self.y() + y, self.width(), self.height())
    }

    #[inline]
    pub fn xywh(&self) -> (i32, i32, u32, u32) {
        (self.raw.x, self.raw.y, self.raw.w as u32, self.raw.h as u32)
    }

    #[inline] pub fn x(&self) -> i32 { self.raw.x }
    #[inline] pub fn y(&self) -> i32 { self.raw.y }
    #[inline] pub fn width(&self) -> u32 { self.raw.w as u32 }
    #[inline] pub fn height(&self) -> u32 { self.raw.h as u32 }

    /// Calculate a minimal rectangle enclosing a set of points.
    ///
    /// Returns `Ok(None)` if there are no points, or no points within the clipping rectangle.
    /// Returns an error if the resulting rectangle's dimensions are too large for the points.
    pub fn from_enclose_points(points: &[Point], clip: Option<Rect>) 
            -> Result<Rect, String> {
        let mut out = unsafe { mem::uninitialized() };

        let clip_ptr = match clip.as_ref() {
            Some(r) => r.raw(),
            None => ::std::ptr::null()
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
            Rect::from_ll(out)
        } else {
            Err(get_error())
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
    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        let mut out = unsafe { mem::uninitialized() };

        let success = unsafe {
            ll::SDL_IntersectRect(self.raw(), other.raw(), &mut out) != 0
        };

        if success {
            Some(Rect::from_ll(out).unwrap())
        } else {
            None
        }
    }

    /// Calculate the union of two rectangles. 
    /// The bitwise OR operator `|` can also be used.
    pub fn union(&self, other: &Rect) -> Rect {
        let mut out = unsafe { mem::uninitialized() };

        unsafe {
            // If `self` and `other` are both empty, `out` remains uninitialized.
            // Because empty rectangles aren't allowed in Rect, we don't need to worry about this.
            ll::SDL_UnionRect(self.raw(), other.raw(), &mut out)
        };

        Rect::from_ll(out).unwrap()
    }

    /// Calculate the intersection of a rectangle and line segment. return points of intersection.
    pub fn intersect_line(&self, start: Point, end: Point) -> Option<(Point, Point)> {
        let (mut start_x, mut start_y) = start.xy();
        let (mut end_x, mut end_y) = end.xy();

        let intersected = unsafe {
            ll::SDL_IntersectRectAndLine(self.raw(), &mut start_x, &mut start_y, &mut end_x, &mut end_y) != 0
        };

        if intersected {
            Some((Point::new(start_x, start_y), Point::new(end_x, end_y)))
        } else {
            None
        }
    }
}

/// Intersect
impl BitAnd<Rect> for Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Rect) -> Option<Rect> { self.intersect(&rhs) }
}

/// Intersect
impl<'a> BitAnd<&'a Rect> for Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: &Rect) -> Option<Rect> { self.intersect(rhs) }
}

/// Intersect
impl<'a> BitAnd<Rect> for &'a Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Rect) -> Option<Rect> { self.intersect(&rhs) }
}

/// Intersect
impl<'a> BitAnd<&'a Rect> for &'a Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: &Rect) -> Option<Rect> { self.intersect(rhs) }
}

/// Union
impl BitOr<Rect> for Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect { self.union(&rhs) }
}

/// Union
impl<'a> BitOr<&'a Rect> for Rect {
    type Output = Rect;
    fn bitor(self, rhs: &Rect) -> Rect { self.union(rhs) }
}

/// Union
impl<'a> BitOr<Rect> for &'a Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect { self.union(&rhs) }
}

/// Union
impl<'a> BitOr<&'a Rect> for &'a Rect {
    type Output = Rect;
    fn bitor(self, rhs: &Rect) -> Rect { self.union(rhs) }
}

#[cfg(test)]
mod test {
    use super::{Rect, Point};

    fn rect(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Rect::new(x, y, w, h).unwrap()
    }

    #[test]
    fn test_from_enclose_points() {
        use std::i32;

        assert_eq!(Rect::from_enclose_points(&[Point::new(2, 4), Point::new(5,9)], None),
                   Ok(Some(rect(2, 4, 4, 6))));
        assert_eq!(Rect::from_enclose_points(&[Point::new(0, 0), Point::new(10,10)],
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
