use libc::c_int;
use sdl::SDL_bool;

/// A structure that defines a two dimensional point.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub struct SDL_Point {
    pub x: c_int,
    pub y: c_int
}

/// A structure that defines a rectangle, with the origin at the upper left.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int
}

extern "C" {
    pub fn SDL_HasIntersection(A: *const SDL_Rect, B: *const SDL_Rect) -> SDL_bool;
    pub fn SDL_IntersectRect(A: *const SDL_Rect, B: *const SDL_Rect, result: *mut SDL_Rect) -> SDL_bool;
    pub fn SDL_UnionRect(A: *const SDL_Rect, B: *const SDL_Rect, result: *mut SDL_Rect);
    pub fn SDL_EnclosePoints(points: *const SDL_Point, count: c_int, clip: *const SDL_Rect, result: *mut SDL_Rect) -> SDL_bool;
    pub fn SDL_IntersectRectAndLine(rect: *const SDL_Rect, X1: *mut c_int, Y1: *mut c_int, X2: *mut c_int, Y2: *mut c_int) -> SDL_bool;
}
