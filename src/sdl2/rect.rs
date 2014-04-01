use std::cast;
use std::libc::c_int;
use std::ptr;

#[deriving(Eq)]
#[deriving(Clone)]
pub struct Point{
    pub x: i32,
    pub y: i32
}

#[deriving(Eq)]
#[deriving(Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32 
}

#[allow(non_camel_case_types)]
pub mod ll {

    use std::libc::{c_int};
    use rect::Rect;
    use rect::Point;

    pub type SDL_Rect = Rect;
    pub type SDL_Point = Point;
    pub type SDL_bool = c_int;
    
    extern "C" {
        pub fn SDL_HasIntersection(A: *SDL_Rect, B: *SDL_Rect) -> SDL_bool;
        pub fn SDL_IntersectRect(A: *SDL_Rect, B: *SDL_Rect, result: *SDL_Rect) -> SDL_bool;
        pub fn SDL_UnionRect(A: *SDL_Rect, B: *SDL_Rect, result: *SDL_Rect);
        pub fn SDL_EnclosePoints(points: *SDL_Point, count: c_int, clip: *SDL_Rect, result: *SDL_Rect) -> SDL_bool;
        pub fn SDL_IntersectRectAndLine(rect: *SDL_Rect, X1: *c_int, Y1: *c_int, X2: *c_int, Y2: *c_int) -> SDL_bool;
    }
}

pub fn Point(x: i32, y: i32) -> Point {
    Point { x: x, y: y }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

pub fn Rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect { x: x, y: y, w: w, h: h }
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

    pub fn enclose_points(points: &[Point], clip: Option<Rect>) -> Option<Rect> {
        let out: Rect = Rect::new(0, 0, 0, 0);

        let result = unsafe {
            ll::SDL_EnclosePoints(
                cast::transmute(points.as_ptr()),
                points.len() as c_int,
                match clip { Some(ref rect) => cast::transmute(rect), None => ptr::null() },
                &out
            ) == 0
        };

        if result {
            Some(out)
        } else {
            None
        }
    }

    pub fn empty(&self) -> bool {
        (self.w <= 0) || (self.h <= 0)
    }

    pub fn has_intersection(&self, other: &Rect) -> bool {
        unsafe {
            ll::SDL_HasIntersection(self, other) == 0
        }
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let out: Rect = Rect::new(0, 0, 0, 0);

        let result = unsafe {
            ll::SDL_IntersectRect(self, other, &out) == 0
        };

        if result {
            Some(out)
        } else {
            None
        }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let out: Rect = Rect::new(0, 0, 0, 0);

        unsafe {
            ll::SDL_UnionRect(self, other, &out)
        };

        out
    }

    pub fn intersect_line(&self, start: &Point, end: &Point) -> Option<(Point, Point)> {
        let out_start: Point = start.clone();
        let out_end: Point = end.clone();

        let result = unsafe {
            ll::SDL_IntersectRectAndLine(self, &out_start.x, &out_start.y, &out_end.x, &out_end.y) == 0
        };

        if result {
            Some((out_start, out_end))
        } else {
            None
        }
    }
}
