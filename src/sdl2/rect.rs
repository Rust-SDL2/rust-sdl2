//! Rectangles and points.

use crate::sys;
use std::convert::{AsMut, AsRef};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{
    Add, AddAssign, BitAnd, BitOr, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub,
    SubAssign,
};
use std::ptr;

/// The maximal integer value that can be used for rectangles.
///
/// This value is smaller than strictly needed, but is useful in ensuring that
/// rect sizes will never have to be truncated when clamping.
pub fn max_int_value() -> u32 {
    i32::MAX as u32 / 2
}

/// The minimal integer value that can be used for rectangle positions
/// and points.
///
/// This value is needed, because otherwise the width of a rectangle created
/// from a point would be able to exceed the maximum width.
pub fn min_int_value() -> i32 {
    i32::MIN / 2
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

fn clamped_mul(a: i32, b: i32) -> i32 {
    match a.checked_mul(b) {
        Some(val) => val,
        None => {
            if (a < 0) ^ (b < 0) {
                min_int_value()
            } else {
                max_int_value() as i32
            }
        }
    }
}

fn clamp_f32_size(val: f32) -> f32 {
    if val <= 0.0 {
        1.0
    } else {
        val
    }
}

/// A (non-empty) rectangle.
///
/// The width and height of a `Rect` must always be strictly positive (never
/// zero).  In cases where empty rects may need to represented, it is
/// recommended to use `Option<Rect>`, with `None` representing an empty
/// rectangle (see, for example, the output of the
/// [`intersection`](#method.intersection) method).
// Uses repr(transparent) to allow pointer casting between Rect and SDL_Rect (see
// `Rect::raw_slice`)
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Rect {
    raw: sys::SDL_Rect,
}

impl ::std::fmt::Debug for Rect {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Rect")
            .field("x", &self.raw.x)
            .field("y", &self.raw.y)
            .field("w", &self.raw.w)
            .field("h", &self.raw.h)
            .finish()
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Rect) -> bool {
        self.raw.x == other.raw.x
            && self.raw.y == other.raw.y
            && self.raw.w == other.raw.w
            && self.raw.h == other.raw.h
    }
}

impl Eq for Rect {}

impl Hash for Rect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.x.hash(state);
        self.raw.y.hash(state);
        self.raw.w.hash(state);
        self.raw.h.hash(state);
    }
}

impl Rect {
    /// Creates a new rectangle from the given values.
    ///
    /// The width and height are clamped to ensure that the right and bottom
    /// sides of the rectangle does not exceed i32::MAX (the value
    /// 2147483647, the maximal positive size of an i32).  This means that the
    /// rect size will behave oddly if you move it very far to the right or
    /// downwards on the screen.
    ///
    /// `Rect`s must always be non-empty, so a `width` and/or `height` argument
    /// of 0 will be replaced with 1.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        let raw = sys::SDL_Rect {
            x: clamp_position(x),
            y: clamp_position(y),
            w: clamp_size(width) as i32,
            h: clamp_size(height) as i32,
        };
        Rect { raw }
    }

    /// Creates a new rectangle centered on the given position.
    ///
    /// The width and height are clamped to ensure that the right and bottom
    /// sides of the rectangle does not exceed i32::MAX (the value
    /// 2147483647, the maximal positive size of an i32).  This means that the
    /// rect size will behave oddly if you move it very far to the right or
    /// downwards on the screen.
    ///
    /// `Rect`s must always be non-empty, so a `width` and/or `height` argument
    /// of 0 will be replaced with 1.
    pub fn from_center<P>(center: P, width: u32, height: u32) -> Rect
    where
        P: Into<Point>,
    {
        let raw = sys::SDL_Rect {
            x: 0,
            y: 0,
            w: clamp_size(width) as i32,
            h: clamp_size(height) as i32,
        };
        let mut rect = Rect { raw };
        rect.center_on(center.into());
        rect
    }

    /// The horizontal position of this rectangle.
    #[inline]
    pub fn x(&self) -> i32 {
        self.raw.x
    }

    /// The vertical position of this rectangle.
    #[inline]
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

    /// Returns the width and height of this rectangle.
    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    /// Sets the horizontal position of this rectangle to the given value,
    /// clamped to be less than or equal to i32::MAX / 2.
    pub fn set_x(&mut self, x: i32) {
        self.raw.x = clamp_position(x);
    }

    /// Sets the vertical position of this rectangle to the given value,
    /// clamped to be less than or equal to i32::MAX / 2.
    pub fn set_y(&mut self, y: i32) {
        self.raw.y = clamp_position(y);
    }

    /// Sets the width of this rectangle to the given value,
    /// clamped to be less than or equal to i32::MAX / 2.
    ///
    /// `Rect`s must always be non-empty, so a `width` argument of 0 will be
    /// replaced with 1.
    pub fn set_width(&mut self, width: u32) {
        self.raw.w = clamp_size(width) as i32;
    }

    /// Sets the height of this rectangle to the given value,
    /// clamped to be less than or equal to i32::MAX / 2.
    ///
    /// `Rect`s must always be non-empty, so a `height` argument of 0 will be
    /// replaced with 1.
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

    /// Shifts this rectangle to the left by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// assert_eq!(Rect::new(5, 5, 10, 10).left_shifted(5), Rect::new(0, 5, 10, 10));
    /// ```
    pub fn left_shifted(mut self, offset: i32) -> Rect {
        self.offset(-offset, 0);
        self
    }

    /// Shifts this rectangle to the right by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// assert_eq!(Rect::new(5, 5, 10, 10).right_shifted(5), Rect::new(10, 5, 10, 10));
    /// ```
    pub fn right_shifted(mut self, offset: i32) -> Rect {
        self.offset(offset, 0);
        self
    }

    /// Shifts this rectangle to the top by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// assert_eq!(Rect::new(5, 5, 10, 10).top_shifted(5), Rect::new(5, 0, 10, 10));
    /// ```
    pub fn top_shifted(mut self, offset: i32) -> Rect {
        self.offset(0, -offset);
        self
    }

    /// Shifts this rectangle to the bottom by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// assert_eq!(Rect::new(5, 5, 10, 10).bottom_shifted(5), Rect::new(5, 10, 10, 10));
    /// ```
    pub fn bottom_shifted(mut self, offset: i32) -> Rect {
        self.offset(0, offset);
        self
    }

    /// Returns the center position of this rectangle.
    ///
    /// Note that if the width or height is not a multiple of two,
    /// the center will be rounded down.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{Rect,Point};
    /// let rect = Rect::new(1,0,2,3);
    /// assert_eq!(Point::new(2,1),rect.center());
    /// ```
    pub fn center(&self) -> Point {
        let x = self.raw.x + (self.raw.w / 2);
        let y = self.raw.y + (self.raw.h / 2);
        Point::new(x, y)
    }

    /// Returns the top-left corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{Rect, Point};
    /// let rect = Rect::new(1, 0, 2, 3);
    /// assert_eq!(Point::new(1, 0), rect.top_left());
    /// ```
    pub fn top_left(&self) -> Point {
        Point::new(self.left(), self.top())
    }

    /// Returns the top-right corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{Rect, Point};
    /// let rect = Rect::new(1, 0, 2, 3);
    /// assert_eq!(Point::new(3, 0), rect.top_right());
    /// ```
    pub fn top_right(&self) -> Point {
        Point::new(self.right(), self.top())
    }

    /// Returns the bottom-left corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{Rect, Point};
    /// let rect = Rect::new(1, 0, 2, 3);
    /// assert_eq!(Point::new(1, 3), rect.bottom_left());
    /// ```
    pub fn bottom_left(&self) -> Point {
        Point::new(self.left(), self.bottom())
    }

    /// Returns the bottom-right corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{Rect, Point};
    /// let rect = Rect::new(1, 0, 2, 3);
    /// assert_eq!(Point::new(3, 3), rect.bottom_right());
    /// ```
    pub fn bottom_right(&self) -> Point {
        Point::new(self.right(), self.bottom())
    }

    /// Sets the position of the right side of this rectangle to the given
    /// value, clamped to be less than or equal to i32::MAX / 2.
    pub fn set_right(&mut self, right: i32) {
        self.raw.x = clamp_position(clamp_position(right) - self.raw.w);
    }

    /// Sets the position of the bottom side of this rectangle to the given
    /// value, clamped to be less than or equal to i32::MAX / 2.
    pub fn set_bottom(&mut self, bottom: i32) {
        self.raw.y = clamp_position(clamp_position(bottom) - self.raw.h);
    }

    /// Centers the rectangle on the given point (in place).
    #[inline]
    pub fn center_on<P>(&mut self, point: P)
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        self.raw.x = clamp_position(clamp_position(x) - self.raw.w / 2);
        self.raw.y = clamp_position(clamp_position(y) - self.raw.h / 2);
    }

    /// Centers the rectangle on the given point.
    #[inline]
    pub fn centered_on<P>(mut self, point: P) -> Rect
    where
        P: Into<(i32, i32)>,
    {
        self.center_on(point);
        self
    }

    /// Move this rect and clamp the positions to prevent over/underflow.
    /// This also clamps the size to prevent overflow.
    #[inline]
    pub fn offset(&mut self, x: i32, y: i32) {
        match self.raw.x.checked_add(x) {
            Some(val) => self.raw.x = clamp_position(val),
            None => {
                if x >= 0 {
                    self.raw.x = max_int_value() as i32;
                } else {
                    self.raw.x = i32::MIN;
                }
            }
        }
        match self.raw.y.checked_add(y) {
            Some(val) => self.raw.y = clamp_position(val),
            None => {
                if y >= 0 {
                    self.raw.y = max_int_value() as i32;
                } else {
                    self.raw.y = i32::MIN;
                }
            }
        }
    }

    /// Moves this rect to the given position after clamping the values.
    pub fn reposition<P>(&mut self, point: P)
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        self.raw.x = clamp_position(x);
        self.raw.y = clamp_position(y);
    }

    /// Resizes this rect to the given size after clamping the values.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.raw.w = clamp_size(width) as i32;
        self.raw.h = clamp_size(height) as i32;
    }

    /// Checks whether this rectangle contains a given point.
    ///
    /// Points along the right and bottom edges are not considered to be inside
    /// the rectangle; this way, a 1-by-1 rectangle contains only a single
    /// point.  Another way to look at it is that this method returns true if
    /// and only if the given point would be painted by a call to
    /// [`Renderer::fill_rect`](
    /// ../render/struct.Renderer.html#method.fill_rect).
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::{Rect, Point};
    /// let rect = Rect::new(1, 2, 3, 4);
    /// assert!(rect.contains_point(Point::new(1, 2)));
    /// assert!(!rect.contains_point(Point::new(0, 1)));
    /// assert!(rect.contains_point(Point::new(3, 5)));
    /// assert!(!rect.contains_point(Point::new(4, 6)));
    /// ```
    pub fn contains_point<P>(&self, point: P) -> bool
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        let inside_x = x >= self.left() && x < self.right();
        inside_x && (y >= self.top() && y < self.bottom())
    }

    /// Checks whether this rectangle completely contains another rectangle.
    ///
    /// This method returns true if and only if every point contained by
    /// `other` is also contained by `self`; in other words, if the
    /// intersection of `self` and `other` is equal to `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// let rect = Rect::new(1, 2, 3, 4);
    /// assert!(rect.contains_rect(rect));
    /// assert!(rect.contains_rect(Rect::new(3, 3, 1, 1)));
    /// assert!(!rect.contains_rect(Rect::new(2, 1, 1, 1)));
    /// assert!(!rect.contains_rect(Rect::new(3, 3, 2, 1)));
    /// ```
    pub fn contains_rect(&self, other: Rect) -> bool {
        other.left() >= self.left()
            && other.right() <= self.right()
            && other.top() >= self.top()
            && other.bottom() <= self.bottom()
    }

    /// Returns the underlying C Rect.
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *const sys::SDL_Rect {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> *mut sys::SDL_Rect {
        &mut self.raw
    }

    #[doc(alias = "SDL_Rect")]
    pub fn raw_slice(slice: &[Rect]) -> *const sys::SDL_Rect {
        slice.as_ptr() as *const sys::SDL_Rect
    }

    pub fn from_ll(raw: sys::SDL_Rect) -> Rect {
        Rect::new(raw.x, raw.y, raw.w as u32, raw.h as u32)
    }

    /// Calculate a minimal rectangle enclosing a set of points.
    /// If a clipping rectangle is given, only points that are within it will be
    /// considered.
    #[doc(alias = "SDL_EnclosePoints")]
    pub fn from_enclose_points<R>(points: &[Point], clipping_rect: R) -> Option<Rect>
    where
        R: Into<Option<Rect>>,
    {
        let clipping_rect = clipping_rect.into();

        if points.is_empty() {
            return None;
        }

        let mut out = mem::MaybeUninit::uninit();

        let clip_ptr = match clipping_rect.as_ref() {
            Some(r) => r.raw(),
            None => ptr::null(),
        };

        let result = unsafe {
            sys::SDL_EnclosePoints(
                Point::raw_slice(points),
                points.len() as i32,
                clip_ptr,
                out.as_mut_ptr(),
            ) != sys::SDL_bool::SDL_FALSE
        };

        if result {
            let out = unsafe { out.assume_init() };

            // Return an error if the dimensions are too large.
            Some(Rect::from_ll(out))
        } else {
            None
        }
    }

    /// Determines whether two rectangles intersect.
    ///
    /// Rectangles that share an edge but don't actually overlap are not
    /// considered to intersect.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// let rect = Rect::new(0, 0, 5, 5);
    /// assert!(rect.has_intersection(rect));
    /// assert!(rect.has_intersection(Rect::new(2, 2, 5, 5)));
    /// assert!(!rect.has_intersection(Rect::new(5, 0, 5, 5)));
    /// ```
    #[doc(alias = "SDL_HasIntersection")]
    pub fn has_intersection(&self, other: Rect) -> bool {
        unsafe { sys::SDL_HasIntersection(self.raw(), other.raw()) != sys::SDL_bool::SDL_FALSE }
    }

    /// Calculates the intersection of two rectangles.
    ///
    /// Returns `None` if the two rectangles don't intersect.  Rectangles that
    /// share an edge but don't actually overlap are not considered to
    /// intersect.
    ///
    /// The bitwise AND operator `&` can also be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// let rect = Rect::new(0, 0, 5, 5);
    /// assert_eq!(rect.intersection(rect), Some(rect));
    /// assert_eq!(rect.intersection(Rect::new(2, 2, 5, 5)),
    ///            Some(Rect::new(2, 2, 3, 3)));
    /// assert_eq!(rect.intersection(Rect::new(5, 0, 5, 5)), None);
    /// ```
    #[doc(alias = "SDL_IntersectRect")]
    pub fn intersection(&self, other: Rect) -> Option<Rect> {
        let mut out = mem::MaybeUninit::uninit();

        let success = unsafe {
            sys::SDL_IntersectRect(self.raw(), other.raw(), out.as_mut_ptr())
                != sys::SDL_bool::SDL_FALSE
        };

        if success {
            let out = unsafe { out.assume_init() };
            Some(Rect::from_ll(out))
        } else {
            None
        }
    }

    /// Calculates the union of two rectangles (i.e. the smallest rectangle
    /// that contains both).
    ///
    /// The bitwise OR operator `|` can also be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::Rect;
    /// let rect = Rect::new(0, 0, 5, 5);
    /// assert_eq!(rect.union(rect), rect);
    /// assert_eq!(rect.union(Rect::new(2, 2, 5, 5)), Rect::new(0, 0, 7, 7));
    /// assert_eq!(rect.union(Rect::new(5, 0, 5, 5)), Rect::new(0, 0, 10, 5));
    /// ```
    #[doc(alias = "SDL_UnionRect")]
    pub fn union(&self, other: Rect) -> Rect {
        let mut out = mem::MaybeUninit::uninit();

        unsafe {
            // If `self` and `other` are both empty, `out` remains uninitialized.
            // Because empty rectangles aren't allowed in Rect, we don't need to worry about this.
            sys::SDL_UnionRect(self.raw(), other.raw(), out.as_mut_ptr())
        };

        let out = unsafe { out.assume_init() };

        Rect::from_ll(out)
    }

    /// Calculates the intersection of a rectangle and a line segment and
    /// returns the points of their intersection.
    #[doc(alias = "SDL_IntersectRectAndLine")]
    pub fn intersect_line(&self, start: Point, end: Point) -> Option<(Point, Point)> {
        let (mut start_x, mut start_y) = (start.x(), start.y());
        let (mut end_x, mut end_y) = (end.x(), end.y());

        let intersected = unsafe {
            sys::SDL_IntersectRectAndLine(
                self.raw(),
                &mut start_x,
                &mut start_y,
                &mut end_x,
                &mut end_y,
            ) != sys::SDL_bool::SDL_FALSE
        };

        if intersected {
            Some((Point::new(start_x, start_y), Point::new(end_x, end_y)))
        } else {
            None
        }
    }
}

impl Deref for Rect {
    type Target = sys::SDL_Rect;

    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::Rect;
    /// let rect = Rect::new(2, 3, 4, 5);
    /// assert_eq!(2, rect.x);
    /// ```
    fn deref(&self) -> &sys::SDL_Rect {
        &self.raw
    }
}

impl DerefMut for Rect {
    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::Rect;
    /// let mut rect = Rect::new(2, 3, 4, 5);
    /// rect.x = 60;
    /// assert_eq!(60, rect.x);
    /// ```
    fn deref_mut(&mut self) -> &mut sys::SDL_Rect {
        &mut self.raw
    }
}

impl From<Rect> for sys::SDL_Rect {
    fn from(val: Rect) -> Self {
        val.raw
    }
}

impl From<Rect> for (i32, i32, u32, u32) {
    fn from(val: Rect) -> Self {
        (val.raw.x, val.raw.y, val.raw.w as u32, val.raw.h as u32)
    }
}

impl From<sys::SDL_Rect> for Rect {
    fn from(raw: sys::SDL_Rect) -> Rect {
        Rect { raw }
    }
}

impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, width, height): (i32, i32, u32, u32)) -> Rect {
        Rect::new(x, y, width, height)
    }
}

impl AsRef<sys::SDL_Rect> for Rect {
    fn as_ref(&self) -> &sys::SDL_Rect {
        &self.raw
    }
}

impl AsMut<sys::SDL_Rect> for Rect {
    fn as_mut(&mut self) -> &mut sys::SDL_Rect {
        &mut self.raw
    }
}

// Intersection
impl BitAnd<Rect> for Rect {
    type Output = Option<Rect>;
    #[doc(alias = "SDL_Point")]
    fn bitand(self, rhs: Rect) -> Option<Rect> {
        self.intersection(rhs)
    }
}

// Union
impl BitOr<Rect> for Rect {
    type Output = Rect;
    fn bitor(self, rhs: Rect) -> Rect {
        self.union(rhs)
    }
}

/// Immutable point type, consisting of x and y.
// Uses repr(transparent) to allow pointer casting between Point and SDL_Point (see
// `Point::raw_slice`)
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Point {
    raw: sys::SDL_Point,
}

impl ::std::fmt::Debug for Point {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Point")
            .field("x", &self.raw.x)
            .field("y", &self.raw.y)
            .finish()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.raw.x == other.raw.x && self.raw.y == other.raw.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.x.hash(state);
        self.raw.y.hash(state);
    }
}

impl Deref for Point {
    type Target = sys::SDL_Point;

    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::Point;
    /// let point = Point::new(2, 3);
    /// assert_eq!(2, point.x);
    /// ```
    fn deref(&self) -> &sys::SDL_Point {
        &self.raw
    }
}

impl DerefMut for Point {
    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::Point;
    /// let mut point = Point::new(2, 3);
    /// point.x = 4;
    /// assert_eq!(4, point.x);
    /// ```
    fn deref_mut(&mut self) -> &mut sys::SDL_Point {
        &mut self.raw
    }
}

impl AsRef<sys::SDL_Point> for Point {
    fn as_ref(&self) -> &sys::SDL_Point {
        &self.raw
    }
}

impl AsMut<sys::SDL_Point> for Point {
    fn as_mut(&mut self) -> &mut sys::SDL_Point {
        &mut self.raw
    }
}

impl From<sys::SDL_Point> for Point {
    fn from(prim: sys::SDL_Point) -> Point {
        Point { raw: prim }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point::new(x, y)
    }
}

impl From<Point> for sys::SDL_Point {
    fn from(val: Point) -> Self {
        val.raw
    }
}

impl From<Point> for (i32, i32) {
    fn from(val: Point) -> Self {
        (val.x(), val.y())
    }
}

impl Point {
    /// Creates a new point from the given coordinates.
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            raw: sys::SDL_Point {
                x: clamp_position(x),
                y: clamp_position(y),
            },
        }
    }

    pub fn from_ll(raw: sys::SDL_Point) -> Point {
        Point::new(raw.x, raw.y)
    }

    #[doc(alias = "SDL_Point")]
    pub fn raw_slice(slice: &[Point]) -> *const sys::SDL_Point {
        slice.as_ptr() as *const sys::SDL_Point
    }
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *const sys::SDL_Point {
        &self.raw
    }

    /// Returns a new point by shifting this point's coordinates by the given
    /// x and y values.
    pub fn offset(self, x: i32, y: i32) -> Point {
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

    /// Returns a new point by multiplying this point's coordinates by the
    /// given scale factor.
    pub fn scale(self, f: i32) -> Point {
        Point::new(clamped_mul(self.raw.x, f), clamped_mul(self.raw.y, f))
    }

    /// Returns the x-coordinate of this point.
    pub fn x(self) -> i32 {
        self.raw.x
    }

    /// Returns the y-coordinate of this point.
    pub fn y(self) -> i32 {
        self.raw.y
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        self.offset(rhs.x(), rhs.y())
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.raw.x = clamp_position(self.x() + rhs.x());
        self.raw.y = clamp_position(self.y() + rhs.y());
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point::new(-self.x(), -self.y())
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        self.offset(-rhs.x(), -rhs.y())
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.raw.x = clamp_position(self.x() - rhs.x());
        self.raw.y = clamp_position(self.y() - rhs.y());
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        self.scale(rhs)
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.raw.x = clamped_mul(self.x(), rhs);
        self.raw.y = clamped_mul(self.y(), rhs);
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        Point::new(self.x() / rhs, self.y() / rhs)
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, rhs: i32) {
        self.raw.x /= rhs;
        self.raw.y /= rhs;
    }
}

impl std::iter::Sum for Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Point::new(0, 0), Point::add)
    }
}

/// A (non-empty) rectangle with float precision.
///
/// The width and height of a `FRect` must always be strictly positive (never
/// zero). In cases where empty rects may need to be represented, it is
/// recommended to use `Option<FRect>`, with `None` representing an empty
/// rectangle (see, for example, the output of the
/// [`intersection`](#method.intersection) method).
#[derive(Clone, Copy)]
pub struct FRect {
    raw: sys::SDL_FRect,
}

impl ::std::fmt::Debug for FRect {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(
            fmt,
            "FRect {{ x: {}, y: {}, w: {}, h: {} }}",
            self.raw.x, self.raw.y, self.raw.w, self.raw.h
        );
    }
}

impl PartialEq for FRect {
    fn eq(&self, other: &FRect) -> bool {
        self.raw.x == other.raw.x
            && self.raw.y == other.raw.y
            && self.raw.w == other.raw.w
            && self.raw.h == other.raw.h
    }
}

impl FRect {
    /// Creates a new rectangle with float precision from the given values.
    ///
    /// `FRect`s must always be non-empty, so a `width` and/or `height` argument
    /// of 0 or less will be replaced with 1.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> FRect {
        let raw = sys::SDL_FRect {
            x,
            y,
            w: clamp_f32_size(width),
            h: clamp_f32_size(height),
        };
        FRect { raw }
    }

    /// Creates a new rectangle with float precision centered on the given position.
    ///
    /// `FRect`s must always be non-empty, so a `width` and/or `height` argument
    /// of 0 or less will be replaced with 1.
    pub fn from_center<P>(center: P, width: f32, height: f32) -> FRect
    where
        P: Into<FPoint>,
    {
        let raw = sys::SDL_FRect {
            x: 0.0,
            y: 0.0,
            w: clamp_f32_size(width),
            h: clamp_f32_size(height),
        };
        let mut rect = FRect { raw };
        rect.center_on(center.into());
        rect
    }

    /// The horizontal position of this rectangle.
    #[inline]
    pub fn x(&self) -> f32 {
        self.raw.x
    }

    /// The vertical position of this rectangle.
    #[inline]
    pub fn y(&self) -> f32 {
        self.raw.y
    }

    /// The width of this rectangle.
    pub fn width(&self) -> f32 {
        self.raw.w as f32
    }

    /// The height of this rectangle.
    pub fn height(&self) -> f32 {
        self.raw.h as f32
    }

    /// Returns the width and height of this rectangle.
    pub fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }

    /// Sets the horizontal position of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_x(&mut self, x: f32) {
        self.raw.x = x;
    }

    /// Sets the vertical position of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    pub fn set_y(&mut self, y: f32) {
        self.raw.y = y;
    }

    /// Sets the width of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    ///
    /// `FRect`s must always be non-empty, so a `width` argument of 0 will be
    /// replaced with 1.
    pub fn set_width(&mut self, width: f32) {
        self.raw.w = clamp_f32_size(width);
    }

    /// Sets the height of this rectangle to the given value,
    /// clamped to be less than or equal to i32::max_value() / 2.
    ///
    /// `FRect`s must always be non-empty, so a `height` argument of 0 will be
    /// replaced with 1.
    pub fn set_height(&mut self, height: f32) {
        self.raw.h = clamp_f32_size(height);
    }

    /// Returns the x-position of the left side of this rectangle.
    pub fn left(&self) -> f32 {
        self.raw.x
    }

    /// Returns the x-position of the right side of this rectangle.
    pub fn right(&self) -> f32 {
        self.raw.x + self.raw.w
    }

    /// Returns the y-position of the top side of this rectangle.
    pub fn top(&self) -> f32 {
        self.raw.y
    }

    /// Returns the y-position of the bottom side of this rectangle.
    pub fn bottom(&self) -> f32 {
        self.raw.y + self.raw.h
    }

    /// Shifts this rectangle to the left by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// assert_eq!(FRect::new(0.0, 0.0, 10.0, 10.0).left_shifted(5.0), FRect::new(-5.0, 0.0, 10.0, 10.0));
    /// ```
    pub fn left_shifted(mut self, offset: f32) -> FRect {
        self.offset(-offset, self.y());
        self
    }

    /// Shifts this rectangle to the right by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// assert_eq!(FRect::new(0.0, 0.0, 10.0, 10.0).right_shifted(5.0), FRect::new(5.0, 0.0, 10.0, 10.0));
    /// ```
    pub fn right_shifted(mut self, offset: f32) -> FRect {
        self.offset(offset, self.y());
        self
    }

    /// Shifts this rectangle to the top by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// assert_eq!(FRect::new(0.0, 0.0, 10.0, 10.0).top_shifted(5.00), FRect::new(0.0, -5.0, 10.0, 10.0));
    /// ```
    pub fn top_shifted(mut self, offset: f32) -> FRect {
        self.offset(self.x(), -offset);
        self
    }

    /// Shifts this rectangle to the bottom by `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// assert_eq!(FRect::new(0.0, 0.0, 10.0, 10.0).bottom_shifted(5.0), FRect::new(0.0, 5.0, 10.0, 10.0));
    /// ```
    pub fn bottom_shifted(mut self, offset: f32) -> FRect {
        self.offset(self.x(), offset);
        self
    }

    /// Returns the center position of this rectangle.
    ///
    /// Note that if the width or height is not a multiple of two,
    /// the center will be rounded down.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{FRect, FPoint};
    /// let rect = FRect::new(1.0, 0.0, 2.0, 3.0);
    /// assert_eq!(FPoint::new(2.0, 1.5), rect.center());
    /// ```
    pub fn center(&self) -> FPoint {
        let x = self.raw.x + (self.raw.w / 2.0);
        let y = self.raw.y + (self.raw.h / 2.0);
        FPoint::new(x, y)
    }

    /// Returns the top-left corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{FRect, FPoint};
    /// let rect = FRect::new(1.0, 0.0, 2.0, 3.0);
    /// assert_eq!(FPoint::new(1.0, 0.0), rect.top_left());
    /// ```
    pub fn top_left(&self) -> FPoint {
        FPoint::new(self.left(), self.top())
    }

    /// Returns the top-right corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{FRect, FPoint};
    /// let rect = FRect::new(1.0, 0.0, 2.0, 3.0);
    /// assert_eq!(FPoint::new(3.0, 0.0), rect.top_right());
    /// ```
    pub fn top_right(&self) -> FPoint {
        FPoint::new(self.right(), self.top())
    }

    /// Returns the bottom-left corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{FRect, FPoint};
    /// let rect = FRect::new(1.0, 0.0, 2.0, 3.0);
    /// assert_eq!(FPoint::new(1.0, 3.0), rect.bottom_left());
    /// ```
    pub fn bottom_left(&self) -> FPoint {
        FPoint::new(self.left(), self.bottom())
    }

    /// Returns the bottom-right corner of this rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use sdl2::rect::{FRect, FPoint};
    /// let rect = FRect::new(1.0, 0.0, 2.0, 3.0);
    /// assert_eq!(FPoint::new(3.0, 3.0), rect.bottom_right());
    /// ```
    pub fn bottom_right(&self) -> FPoint {
        FPoint::new(self.right(), self.bottom())
    }

    /// Sets the position of the right side of this rectangle to the given
    /// value, clamped to be greater than 0.
    pub fn set_right(&mut self, right: f32) {
        self.raw.x = clamp_f32_size(clamp_f32_size(right) - self.raw.w);
    }

    /// Sets the position of the bottom side of this rectangle to the given
    /// value, clamped to be greater than 0.
    pub fn set_bottom(&mut self, bottom: f32) {
        self.raw.y = clamp_f32_size(clamp_f32_size(bottom) - self.raw.h);
    }

    /// Centers the rectangle on the given point (in place).
    #[inline]
    pub fn center_on<P>(&mut self, point: P)
    where
        P: Into<(f32, f32)>,
    {
        let (x, y) = point.into();
        self.raw.x = x - self.raw.w / 2.0;
        self.raw.y = y - self.raw.h / 2.0;
    }

    /// Centers the rectangle on the given point.
    #[inline]
    pub fn centered_on<P>(mut self, point: P) -> FRect
    where
        P: Into<(f32, f32)>,
    {
        self.center_on(point);
        self
    }

    /// Move this rect.
    #[inline]
    pub fn offset(&mut self, x: f32, y: f32) {
        self.raw.x += x;
        self.raw.y += y;
    }

    /// Moves this rect to the given position.
    pub fn reposition<P>(&mut self, point: P)
    where
        P: Into<(f32, f32)>,
    {
        let (x, y) = point.into();
        self.raw.x = x;
        self.raw.y = y;
    }

    /// Resizes this rect to the given size after clamping the values.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.raw.w = clamp_f32_size(width);
        self.raw.h = clamp_f32_size(height);
    }

    /// Checks whether this rectangle contains a given point.
    ///
    /// Points along the right and bottom edges are not considered to be inside
    /// the rectangle. Another way to look at it is that this method returns true if
    /// and only if the given point would be painted by a call to
    /// [`Renderer::fill_frect`](
    /// ../render/struct.Renderer.html#method.fill_frect).
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::{FRect, FPoint};
    /// let rect = FRect::new(1.0, 2.0, 3.0, 4.0);
    /// assert!(rect.contains_point(FPoint::new(1.0, 2.0)));
    /// assert!(!rect.contains_point(FPoint::new(0.0, 1.0)));
    /// assert!(rect.contains_point(FPoint::new(3.0, 5.0)));
    /// assert!(!rect.contains_point(FPoint::new(4.0, 6.0)));
    /// ```
    pub fn contains_point<P>(&self, point: P) -> bool
    where
        P: Into<(f32, f32)>,
    {
        let (x, y) = point.into();
        let inside_x = x >= self.left() && x < self.right();
        inside_x && (y >= self.top() && y < self.bottom())
    }

    /// Checks whether this rectangle completely contains another rectangle.
    ///
    /// This method returns true if and only if every point contained by
    /// `other` is also contained by `self`; in other words, if the
    /// intersection of `self` and `other` is equal to `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// let rect = FRect::new(1.0, 2.0, 3.0, 4.0);
    /// assert!(rect.contains_rect(rect));
    /// assert!(rect.contains_rect(FRect::new(3.0, 3.0, 1.0, 1.0)));
    /// assert!(!rect.contains_rect(FRect::new(2.0, 1.0, 1.0, 1.0)));
    /// assert!(!rect.contains_rect(FRect::new(3.0, 3.0, 2.0, 1.0)));
    /// ```
    pub fn contains_rect(&self, other: FRect) -> bool {
        other.left() >= self.left()
            && other.right() <= self.right()
            && other.top() >= self.top()
            && other.bottom() <= self.bottom()
    }

    /// Returns the underlying C FRect.
    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *const sys::SDL_FRect {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> *mut sys::SDL_FRect {
        self.raw() as *mut _
    }

    #[doc(alias = "SDL_FRect")]
    pub fn raw_slice(slice: &[FRect]) -> *const sys::SDL_FRect {
        slice.as_ptr() as *const sys::SDL_FRect
    }

    pub fn from_ll(raw: sys::SDL_FRect) -> FRect {
        FRect::new(raw.x, raw.y, raw.w, raw.h)
    }

    /// Calculate a minimal rectangle enclosing a set of points.
    /// If a clipping rectangle is given, only points that are within it will be
    /// considered.
    #[doc(alias = "SDL_EncloseFPoints")]
    pub fn from_enclose_points<R: Into<Option<FRect>>>(
        points: &[FPoint],
        clipping_rect: R,
    ) -> Option<FRect>
    where
        R: Into<Option<FRect>>,
    {
        let clipping_rect = clipping_rect.into();

        if points.is_empty() {
            return None;
        }

        let mut out = mem::MaybeUninit::uninit();

        let clip_ptr = match clipping_rect.as_ref() {
            Some(r) => r.raw(),
            None => ptr::null(),
        };

        let result = unsafe {
            sys::SDL_EncloseFPoints(
                FPoint::raw_slice(points),
                points.len() as i32,
                clip_ptr,
                out.as_mut_ptr(),
            ) != sys::SDL_bool::SDL_FALSE
        };

        if result {
            let out = unsafe { out.assume_init() };

            // Return an error if the dimensions are too large.
            Some(FRect::from_ll(out))
        } else {
            None
        }
    }

    /// Determines whether two rectangles intersect.
    ///
    /// Rectangles that share an edge but don't actually overlap are not
    /// considered to intersect.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// let rect = FRect::new(0.0, 0.0, 5.0, 5.0);
    /// assert!(rect.has_intersection(rect));
    /// assert!(rect.has_intersection(FRect::new(2.0, 2.0, 5.0, 5.0)));
    /// assert!(!rect.has_intersection(FRect::new(5.0, 0.0, 5.0, 5.0)));
    /// ```
    #[doc(alias = "SDL_HasIntersectionF")]
    pub fn has_intersection(&self, other: FRect) -> bool {
        unsafe { sys::SDL_HasIntersectionF(self.raw(), other.raw()) != sys::SDL_bool::SDL_FALSE }
    }

    /// Calculates the intersection of two rectangles.
    ///
    /// Returns `None` if the two rectangles don't intersect.  Rectangles that
    /// share an edge but don't actually overlap are not considered to
    /// intersect.
    ///
    /// The bitwise AND operator `&` can also be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// let rect = FRect::new(0.0, 0.0, 5.0, 5.0);
    /// assert_eq!(rect.intersection(rect), Some(rect));
    /// assert_eq!(rect.intersection(FRect::new(2.0, 2.0, 5.0, 5.0)),
    ///            Some(FRect::new(2.0, 2.0, 3.0, 3.0)));
    /// assert_eq!(rect.intersection(FRect::new(5.0, 0.0, 5.0, 5.0)), None);
    /// ```
    #[doc(alias = "SDL_IntersectFRect")]
    pub fn intersection(&self, other: FRect) -> Option<FRect> {
        let mut out = mem::MaybeUninit::uninit();

        let success = unsafe {
            sys::SDL_IntersectFRect(self.raw(), other.raw(), out.as_mut_ptr())
                != sys::SDL_bool::SDL_FALSE
        };

        if success {
            let out = unsafe { out.assume_init() };
            Some(FRect::from_ll(out))
        } else {
            None
        }
    }

    /// Calculates the union of two rectangles (i.e. the smallest rectangle
    /// that contains both).
    ///
    /// The bitwise OR operator `|` can also be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl2::rect::FRect;
    /// let rect = FRect::new(0.0, 0.0, 5.0, 5.0);
    /// assert_eq!(rect.union(rect), rect);
    /// assert_eq!(rect.union(FRect::new(2.0, 2.0, 5.0, 5.0)), FRect::new(0.0, 0.0, 7.0, 7.0));
    /// assert_eq!(rect.union(FRect::new(5.0, 0.0, 5.0, 5.0)), FRect::new(0.0, 0.0, 10.0, 5.0));
    /// ```
    #[doc(alias = "SDL_UnionFRect")]
    pub fn union(&self, other: FRect) -> FRect {
        let mut out = mem::MaybeUninit::uninit();

        unsafe {
            // If `self` and `other` are both empty, `out` remains uninitialized.
            // Because empty rectangles aren't allowed in Rect, we don't need to worry about this.
            sys::SDL_UnionFRect(self.raw(), other.raw(), out.as_mut_ptr())
        };

        let out = unsafe { out.assume_init() };

        FRect::from_ll(out)
    }

    /// Calculates the intersection of a rectangle and a line segment and
    /// returns the points of their intersection.
    #[doc(alias = "SDL_IntersectFRectAndLine")]
    pub fn intersect_line(&self, start: FPoint, end: FPoint) -> Option<(FPoint, FPoint)> {
        let (mut start_x, mut start_y) = (start.x(), start.y());
        let (mut end_x, mut end_y) = (end.x(), end.y());

        let intersected = unsafe {
            sys::SDL_IntersectFRectAndLine(
                self.raw(),
                &mut start_x,
                &mut start_y,
                &mut end_x,
                &mut end_y,
            ) != sys::SDL_bool::SDL_FALSE
        };

        if intersected {
            Some((FPoint::new(start_x, start_y), FPoint::new(end_x, end_y)))
        } else {
            None
        }
    }
}

impl Deref for FRect {
    type Target = sys::SDL_FRect;

    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::FRect;
    /// let rect = FRect::new(2.0, 3.0, 4.0, 5.0);
    /// assert_eq!(2.0, rect.x);
    /// ```
    fn deref(&self) -> &sys::SDL_FRect {
        &self.raw
    }
}

impl DerefMut for FRect {
    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::FRect;
    /// let mut rect = FRect::new(2.0, 3.0, 4.0, 5.0);
    /// rect.x = 60.0;
    /// assert_eq!(60.0, rect.x);
    /// ```
    fn deref_mut(&mut self) -> &mut sys::SDL_FRect {
        &mut self.raw
    }
}

impl Into<sys::SDL_FRect> for FRect {
    fn into(self) -> sys::SDL_FRect {
        self.raw
    }
}

impl Into<(f32, f32, f32, f32)> for FRect {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.raw.x, self.raw.y, self.raw.w, self.raw.h)
    }
}

impl From<sys::SDL_FRect> for FRect {
    fn from(raw: sys::SDL_FRect) -> FRect {
        FRect { raw }
    }
}

impl From<(f32, f32, f32, f32)> for FRect {
    fn from((x, y, width, height): (f32, f32, f32, f32)) -> FRect {
        FRect::new(x, y, width, height)
    }
}

impl AsRef<sys::SDL_FRect> for FRect {
    fn as_ref(&self) -> &sys::SDL_FRect {
        &self.raw
    }
}

impl AsMut<sys::SDL_FRect> for FRect {
    fn as_mut(&mut self) -> &mut sys::SDL_FRect {
        &mut self.raw
    }
}

// Intersection
impl BitAnd<FRect> for FRect {
    type Output = Option<FRect>;
    #[doc(alias = "SDL_FPoint")]
    fn bitand(self, rhs: FRect) -> Option<FRect> {
        self.intersection(rhs)
    }
}

// Union
impl BitOr<FRect> for FRect {
    type Output = FRect;
    fn bitor(self, rhs: FRect) -> FRect {
        self.union(rhs)
    }
}

/// Immutable point type with float precision, consisting of x and y.
#[derive(Copy, Clone)]
pub struct FPoint {
    raw: sys::SDL_FPoint,
}

impl ::std::fmt::Debug for FPoint {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(fmt, "FPoint {{ x: {}, y: {} }}", self.raw.x, self.raw.y);
    }
}

impl PartialEq for FPoint {
    fn eq(&self, other: &FPoint) -> bool {
        self.raw.x == other.raw.x && self.raw.y == other.raw.y
    }
}

impl Deref for FPoint {
    type Target = sys::SDL_FPoint;

    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::FPoint;
    /// let point = FPoint::new(2.0, 3.0);
    /// assert_eq!(2.0, point.x);
    /// ```
    fn deref(&self) -> &sys::SDL_FPoint {
        &self.raw
    }
}

impl DerefMut for FPoint {
    /// # Example
    ///
    /// ```rust
    /// use sdl2::rect::FPoint;
    /// let mut point = FPoint::new(2.0, 3.0);
    /// point.x = 4.0;
    /// assert_eq!(4.0, point.x);
    /// ```
    fn deref_mut(&mut self) -> &mut sys::SDL_FPoint {
        &mut self.raw
    }
}

impl AsRef<sys::SDL_FPoint> for FPoint {
    fn as_ref(&self) -> &sys::SDL_FPoint {
        &self.raw
    }
}

impl AsMut<sys::SDL_FPoint> for FPoint {
    fn as_mut(&mut self) -> &mut sys::SDL_FPoint {
        &mut self.raw
    }
}

impl From<sys::SDL_FPoint> for FPoint {
    fn from(prim: sys::SDL_FPoint) -> FPoint {
        FPoint { raw: prim }
    }
}

impl From<(f32, f32)> for FPoint {
    fn from((x, y): (f32, f32)) -> FPoint {
        FPoint::new(x, y)
    }
}

impl Into<sys::SDL_FPoint> for FPoint {
    fn into(self) -> sys::SDL_FPoint {
        self.raw
    }
}

impl Into<(f32, f32)> for FPoint {
    fn into(self) -> (f32, f32) {
        (self.x(), self.y())
    }
}

impl FPoint {
    /// Creates a new point from the given coordinates.
    pub fn new(x: f32, y: f32) -> FPoint {
        FPoint {
            raw: sys::SDL_FPoint { x, y },
        }
    }

    pub fn from_ll(raw: sys::SDL_FPoint) -> FPoint {
        FPoint::new(raw.x, raw.y)
    }

    #[doc(alias = "SDL_FPoint")]
    pub fn raw_slice(slice: &[FPoint]) -> *const sys::SDL_FPoint {
        slice.as_ptr() as *const sys::SDL_FPoint
    }

    // this can prevent introducing UB until
    // https://github.com/rust-lang/rust-clippy/issues/5953 is fixed
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn raw(&self) -> *const sys::SDL_FPoint {
        &self.raw
    }

    /// Returns a new point by shifting this point's coordinates by the given
    /// x and y values.
    pub fn offset(self, x: f32, y: f32) -> FPoint {
        let x = self.raw.x + x;
        let y = self.raw.y + y;
        FPoint::new(x, y)
    }

    /// Returns a new point by multiplying this point's coordinates by the
    /// given scale factor.
    pub fn scale(self, f: f32) -> FPoint {
        FPoint::new(self.raw.x * f, self.raw.y * f)
    }

    /// Returns the x-coordinate of this point.
    pub fn x(self) -> f32 {
        self.raw.x
    }

    /// Returns the y-coordinate of this point.
    pub fn y(self) -> f32 {
        self.raw.y
    }
}

impl Add for FPoint {
    type Output = FPoint;

    fn add(self, rhs: FPoint) -> FPoint {
        self.offset(rhs.x(), rhs.y())
    }
}

impl AddAssign for FPoint {
    fn add_assign(&mut self, rhs: FPoint) {
        self.raw.x = self.x() + rhs.x();
        self.raw.y = self.y() + rhs.y();
    }
}

impl Neg for FPoint {
    type Output = FPoint;

    fn neg(self) -> FPoint {
        FPoint::new(-self.x(), -self.y())
    }
}

impl Sub for FPoint {
    type Output = FPoint;

    fn sub(self, rhs: FPoint) -> FPoint {
        self.offset(-rhs.x(), -rhs.y())
    }
}

impl SubAssign for FPoint {
    fn sub_assign(&mut self, rhs: FPoint) {
        self.raw.x = self.x() - rhs.x();
        self.raw.y = self.y() - rhs.y();
    }
}

impl Mul<f32> for FPoint {
    type Output = FPoint;

    fn mul(self, rhs: f32) -> FPoint {
        self.scale(rhs)
    }
}

impl MulAssign<f32> for FPoint {
    fn mul_assign(&mut self, rhs: f32) {
        self.raw.x = self.x() * rhs;
        self.raw.y = self.y() * rhs;
    }
}

impl Div<f32> for FPoint {
    type Output = FPoint;

    fn div(self, rhs: f32) -> FPoint {
        FPoint::new(self.x() / rhs, self.y() / rhs)
    }
}

impl DivAssign<f32> for FPoint {
    fn div_assign(&mut self, rhs: f32) {
        self.raw.x /= rhs;
        self.raw.y /= rhs;
    }
}

impl std::iter::Sum for FPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FPoint::new(0.0, 0.0), FPoint::add)
    }
}

#[cfg(test)]
mod test {
    use super::{max_int_value, min_int_value, FPoint, FRect, Point, Rect};

    /// Used to compare "literal" (unclamped) rect values.
    fn tuple(x: i32, y: i32, w: u32, h: u32) -> (i32, i32, u32, u32) {
        (x, y, w, h)
    }

    #[test]
    fn centered() {
        // Tests both center_on and centered_on
        assert_eq!(
            Rect::new(0, 0, 10, 10).centered_on((0, 0)),
            Rect::new(-5, -5, 10, 10)
        );
    }

    #[test]
    fn enclose_points_valid() {
        assert_eq!(
            Some(tuple(2, 4, 4, 6)),
            Rect::from_enclose_points(&[Point::new(2, 4), Point::new(5, 9)], None)
                .map(|r| r.into())
        );
    }

    #[test]
    fn enclose_points_outside_clip_rect() {
        assert_eq!(
            Rect::from_enclose_points(
                &[Point::new(0, 0), Point::new(10, 10)],
                Some(Rect::new(3, 3, 1, 1))
            ),
            None
        );
    }

    #[test]
    fn enclose_points_max_values() {
        // Try to enclose the top-left-most and bottom-right-most points.
        assert_eq!(
            Some(tuple(
                min_int_value(),
                min_int_value(),
                max_int_value(),
                max_int_value()
            )),
            Rect::from_enclose_points(
                &[
                    Point::new(i32::MIN, i32::MIN),
                    Point::new(i32::MAX, i32::MAX)
                ],
                None
            )
            .map(|r| r.into())
        );
    }

    #[test]
    fn has_intersection() {
        let rect = Rect::new(0, 0, 10, 10);
        assert!(rect.has_intersection(Rect::new(9, 9, 10, 10)));
        // edge
        assert!(!rect.has_intersection(Rect::new(10, 10, 10, 10)));
        // out
        assert!(!rect.has_intersection(Rect::new(11, 11, 10, 10)));
    }

    #[test]
    fn intersection() {
        let rect = Rect::new(0, 0, 10, 10);
        assert_eq!(rect & Rect::new(9, 9, 10, 10), Some(Rect::new(9, 9, 1, 1)));
        assert_eq!(rect & Rect::new(11, 11, 10, 10), None);
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
            Rect::new(1, 1, 5, 5).intersect_line(Point::new(0, 0), Point::new(10, 10)),
            Some((Point::new(1, 1), Point::new(5, 5)))
        );
    }

    #[test]
    fn clamp_size_zero() {
        assert_eq!(tuple(0, 0, 1, 1), Rect::new(0, 0, 0, 0).into());
    }

    #[test]
    fn clamp_position_min() {
        assert_eq!(
            tuple(min_int_value(), min_int_value(), 1, 1),
            Rect::new(i32::MIN, i32::MIN, 1, 1).into()
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
            Rect::new(0, 0, i32::MAX as u32, i32::MAX as u32).into()
        )
    }

    #[test]
    fn clamp_position_max() {
        assert_eq!(
            tuple(max_int_value() as i32, max_int_value() as i32, 1, 1),
            Rect::new(max_int_value() as i32 + 1, max_int_value() as i32 + 1, 1, 1).into()
        );
    }

    #[test]
    fn shifted() {
        // Groups all functions into a single assertion
        let rect = Rect::new(5, 5, 10, 10)
            .left_shifted(5)
            .right_shifted(5)
            .top_shifted(5)
            .bottom_shifted(5);
        assert_eq!(rect, Rect::new(5, 5, 10, 10));
    }

    #[test]
    fn rect_into() {
        let test: (i32, i32, u32, u32) = (-11, 5, 50, 20);
        assert_eq!(test, Rect::new(-11, 5, 50, 20).into());
    }

    #[test]
    fn rect_from() {
        assert_eq!(Rect::from((-11, 5, 50, 20)), Rect::new(-11, 5, 50, 20));
    }

    #[test]
    fn point_into() {
        let test: (i32, i32) = (-11, 5);
        assert_eq!(test, Point::new(-11, 5).into());
    }

    #[test]
    fn point_from() {
        let test: (i32, i32) = (-11, 5);
        assert_eq!(test, Point::new(-11, 5).into());
    }

    #[test]
    fn point_add() {
        assert_eq!(Point::new(-5, 7), Point::new(-11, 5) + Point::new(6, 2));
    }

    #[test]
    fn point_add_assign() {
        let mut point = Point::new(-11, 5);
        point += Point::new(6, 2);
        assert_eq!(point, Point::new(-11, 5) + Point::new(6, 2));
    }

    #[test]
    fn point_sub() {
        assert_eq!(Point::new(-17, 3), Point::new(-11, 5) - Point::new(6, 2));
    }

    #[test]
    fn point_sub_assign() {
        let mut point = Point::new(-11, 5);
        point -= Point::new(6, 2);
        assert_eq!(point, Point::new(-11, 5) - Point::new(6, 2));
    }

    #[test]
    fn point_mul() {
        assert_eq!(Point::new(-33, 15), Point::new(-11, 5) * 3);
    }

    #[test]
    fn point_mul_assign() {
        let mut point = Point::new(-11, 5);
        point *= 3;
        assert_eq!(point, Point::new(-11, 5) * 3);
    }

    #[test]
    fn point_mul_clamp() {
        assert_eq!(
            Point::new(0x7fffffff, -0x7fffffff),
            Point::new(-1000000, 5000000) * -3000000
        );
    }

    #[test]
    fn point_mul_assign_clamp() {
        let mut point = Point::new(-1000000, 5000000);
        point *= -3000000;
        assert_eq!(point, Point::new(-1000000, 5000000) * -3000000);
    }

    #[test]
    fn point_div() {
        assert_eq!(Point::new(-3, 1), Point::new(-11, 5) / 3);
    }

    #[test]
    fn point_div_assign() {
        let mut point = Point::new(-11, 5);
        point /= 3;
        assert_eq!(point, Point::new(-11, 5) / 3);
    }

    #[test]
    fn point_sum() {
        let points_sum: Point = vec![Point::new(-11, 5), Point::new(6, 2)].into_iter().sum();
        assert_eq!(Point::new(-5, 7), points_sum);
    }

    #[test]
    fn frect_centered() {
        // Tests both center_on and centered_on
        assert_eq!(
            FRect::new(0.0, 0.0, 10.0, 10.0).centered_on((0.0, 0.0)),
            FRect::new(-5.0, -5.0, 10.0, 10.0)
        );
    }

    #[test]
    fn frect_enclose_points_valid() {
        assert_eq!(
            Some((2.0, 4.0, 4.0, 6.0)),
            FRect::from_enclose_points(&[FPoint::new(2.0, 4.0), FPoint::new(5.0, 9.0)], None)
                .map(|r| r.into())
        );
    }

    #[test]
    fn frect_enclose_points_outside_clip_rect() {
        assert_eq!(
            FRect::from_enclose_points(
                &[FPoint::new(0.0, 0.0), FPoint::new(10.0, 10.0)],
                Some(FRect::new(3.0, 3.0, 1.0, 1.0))
            ),
            None
        );
    }

    #[test]
    fn frect_has_intersection() {
        let rect = FRect::new(0.0, 0.0, 10.0, 10.0);
        assert!(rect.has_intersection(FRect::new(9.0, 9.0, 10.0, 10.0)));
        // edge
        assert!(!rect.has_intersection(FRect::new(10.0, 10.0, 10.0, 10.0)));
        // out
        assert!(!rect.has_intersection(FRect::new(11.0, 11.0, 10.0, 10.0)));
    }

    #[test]
    fn frect_intersection() {
        let rect = FRect::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(
            rect & FRect::new(9.0, 9.0, 10.0, 10.0),
            Some(FRect::new(9.0, 9.0, 1.0, 1.0))
        );
        assert_eq!(rect & FRect::new(11.0, 11.0, 10.0, 10.0), None);
    }

    #[test]
    fn frect_union() {
        assert_eq!(
            FRect::new(0.0, 0.0, 1.0, 1.0) | FRect::new(9.0, 9.0, 1.0, 1.0),
            FRect::new(0.0, 0.0, 10.0, 10.0)
        );
    }

    #[test]
    fn frect_intersect_line() {
        assert_eq!(
            FRect::new(1.0, 1.0, 5.0, 5.0)
                .intersect_line(FPoint::new(0.0, 0.0), FPoint::new(10.0, 10.0)),
            Some((FPoint::new(1.0, 1.0), FPoint::new(5.0, 5.0)))
        );
    }

    #[test]
    fn frect_shifted() {
        // Groups all functions into a single assertion
        let rect = FRect::new(0.0, 0.0, 10.0, 10.0)
            .left_shifted(5.0)
            .right_shifted(5.0)
            .top_shifted(5.0)
            .bottom_shifted(5.0);
        assert_eq!(rect, FRect::new(0.0, 0.0, 10.0, 10.0));
    }

    #[test]
    fn frect_into() {
        let test: (f32, f32, f32, f32) = (-11.0, 5.0, 50.0, 20.0);
        assert_eq!(test, FRect::new(-11.0, 5.0, 50.0, 20.0).into());
    }

    #[test]
    fn frect_from() {
        assert_eq!(
            FRect::from((-11.0, 5.0, 50.0, 20.0)),
            FRect::new(-11.0, 5.0, 50.0, 20.0)
        );
    }

    #[test]
    fn fpoint_into() {
        let test: (f32, f32) = (-11.0, 5.0);
        assert_eq!(test, FPoint::new(-11.0, 5.0).into());
    }

    #[test]
    fn fpoint_from() {
        let test: (f32, f32) = (-11.0, 5.0);
        assert_eq!(test, FPoint::new(-11.0, 5.0).into());
    }

    #[test]
    fn fpoint_add() {
        assert_eq!(
            FPoint::new(-5.0, 7.0),
            FPoint::new(-11.0, 5.0) + FPoint::new(6.0, 2.0)
        );
    }

    #[test]
    fn fpoint_add_assign() {
        let mut point = FPoint::new(-11.0, 5.0);
        point += FPoint::new(6.0, 2.0);
        assert_eq!(point, FPoint::new(-11.0, 5.0) + FPoint::new(6.0, 2.0));
    }

    #[test]
    fn fpoint_sub() {
        assert_eq!(
            FPoint::new(-17.0, 3.0),
            FPoint::new(-11.0, 5.0) - FPoint::new(6.0, 2.0)
        );
    }

    #[test]
    fn fpoint_sub_assign() {
        let mut point = FPoint::new(-11.0, 5.0);
        point -= FPoint::new(6.0, 2.0);
        assert_eq!(point, FPoint::new(-11.0, 5.0) - FPoint::new(6.0, 2.0));
    }

    #[test]
    fn fpoint_mul() {
        assert_eq!(FPoint::new(-33.0, 15.0), FPoint::new(-11.0, 5.0) * 3.0);
    }

    #[test]
    fn fpoint_mul_assign() {
        let mut point = FPoint::new(-11.0, 5.0);
        point *= 3.0;
        assert_eq!(point, FPoint::new(-11.0, 5.0) * 3.0);
    }

    #[test]
    fn fpoint_div() {
        assert_eq!(FPoint::new(-3.0, 1.0), FPoint::new(-9.0, 3.0) / 3.0);
    }

    #[test]
    fn fpoint_div_assign() {
        let mut point = FPoint::new(-11.0, 5.0);
        point /= 3.0;
        assert_eq!(point, FPoint::new(-11.0, 5.0) / 3.0);
    }

    #[test]
    fn fpoint_sum() {
        let points_sum: FPoint = vec![FPoint::new(-11.0, 5.0), FPoint::new(6.0, 2.0)]
            .into_iter()
            .sum();
        assert_eq!(FPoint::new(-5.0, 7.0), points_sum);
    }
}
