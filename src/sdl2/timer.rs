use std::mem;
use libc::{uint32_t, c_void};
use std::raw;

#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{uint32_t, uint64_t, c_void, c_int};

    //SDL_timer.h
    pub type SDL_TimerCallback =
        ::std::option::Option<extern "C" fn(arg1: uint32_t, arg2: *const c_void)
                                            -> uint32_t>;
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
}

pub fn get_ticks() -> uint {
    unsafe { ll::SDL_GetTicks() as uint }
}

pub fn get_performance_counter() -> u64 {
    unsafe { ll::SDL_GetPerformanceCounter() }
}

pub fn get_performance_frequency() -> u64 {
    unsafe { ll::SDL_GetPerformanceFrequency() }
}

pub fn delay(ms: uint) {
    unsafe { ll::SDL_Delay(ms as u32) }
}

pub struct Timer<'a> {
    delay: uint,
    raw: ll::SDL_TimerID,
    closure: raw::Closure,
    remove_on_drop: bool,
}

impl<'a> Timer<'a> {
    pub fn new<'a>(delay: uint, callback: ||: 'a -> uint, remove_on_drop: bool) -> Timer<'a> {
        unsafe {
            let c_param = mem::transmute::<_, raw::Closure>(callback);
            Timer { delay: delay, raw: 0, closure: c_param, remove_on_drop: remove_on_drop }
        }
    }

    pub fn start(&mut self) {
        unsafe {
            let timer_id = ll::SDL_AddTimer(self.delay as u32, Some(c_timer_callback), mem::transmute(&self.closure));
            self.raw = timer_id;
        }
    }

    pub fn remove(&mut self) -> bool {
        let ret = unsafe { ll::SDL_RemoveTimer(self.raw) };
        if self.raw != 0 {
            self.raw = 0
        }
        ret == 1
    }
}

#[unsafe_destructor]
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        if self.remove_on_drop {
            let ret = unsafe { ll::SDL_RemoveTimer(self.raw) };
            if ret != 1 {
                println!("error dropping timer {}, maybe already removed.", self.raw);
            }
        }
    }
}

extern "C" fn c_timer_callback(_interval: uint32_t, param: *const c_void) -> uint32_t {
    let f : &mut || -> uint = unsafe { mem::transmute(param) };
    (*f)() as uint32_t
}
