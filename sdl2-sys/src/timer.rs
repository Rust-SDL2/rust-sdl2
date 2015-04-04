#[cfg(feature = "no_std")]
use core::prelude::*;
use libc::{uint32_t, uint64_t, c_void, c_int};

//SDL_timer.h
pub type SDL_TimerCallback =
    Option<extern "C" fn(arg1: uint32_t, arg2: *const c_void) -> uint32_t>;
pub type SDL_TimerID = c_int;
extern "C" {
    pub fn SDL_GetTicks() -> uint32_t;
    pub fn SDL_GetPerformanceCounter() -> uint64_t;
    pub fn SDL_GetPerformanceFrequency() -> uint64_t;
    pub fn SDL_Delay(ms: uint32_t);

    pub fn SDL_AddTimer(interval: uint32_t, callback: SDL_TimerCallback,
                        param: *const c_void) -> SDL_TimerID;
    pub fn SDL_RemoveTimer(id: SDL_TimerID) -> c_int;
}
