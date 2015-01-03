#![doc(hidden)]

use libc::{uint8_t, c_char, c_int};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_version {
    pub major: uint8_t,
    pub minor: uint8_t,
    pub patch: uint8_t,
}
extern "C" {
    pub fn SDL_GetVersion(ver: *mut SDL_version);
    pub fn SDL_GetRevision() -> *const c_char;
    pub fn SDL_GetRevisionNumber() -> c_int;
}
