use libc::c_int;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WindowPos {
    Undefined,
    Centered,
    Positioned(i32),
}

impl From<i32> for WindowPos {
    fn from(pos: i32) -> Self {
        WindowPos::Positioned(pos)
    }
}

pub(in crate::video) fn to_ll_windowpos(pos: WindowPos) -> c_int {
    match pos {
        WindowPos::Undefined => sys::SDL_WINDOWPOS_UNDEFINED_MASK as c_int,
        WindowPos::Centered => sys::SDL_WINDOWPOS_CENTERED_MASK as c_int,
        WindowPos::Positioned(x) => x as c_int,
    }
}
