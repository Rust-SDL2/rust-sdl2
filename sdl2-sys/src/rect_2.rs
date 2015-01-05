use libc::c_int;

/// A structure that defines a two dimensional point.
#[derive(PartialEq, Clone, Show, Copy)]
#[repr(C)]
pub struct SDL_Point {
    pub x: i32,
    pub y: i32
}

/// A structure that defines a rectangle, with the origin at the upper left.
#[derive(PartialEq, Clone, Show, Copy)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

pub type SDL_bool = c_int;

extern "C" {
    pub fn SDL_HasIntersection(A: *const SDL_Rect, B: *const SDL_Rect) -> SDL_bool;
    pub fn SDL_IntersectRect(A: *const SDL_Rect, B: *const SDL_Rect, result: *const SDL_Rect) -> SDL_bool;
    pub fn SDL_UnionRect(A: *const SDL_Rect, B: *const SDL_Rect, result: *const SDL_Rect);
    pub fn SDL_EnclosePoints(points: *const SDL_Point, count: c_int, clip: *const SDL_Rect, result: *const SDL_Rect) -> SDL_bool;
    pub fn SDL_IntersectRectAndLine(rect: *const SDL_Rect, X1: *const c_int, Y1: *const c_int, X2: *const c_int, Y2: *const c_int) -> SDL_bool;
}
