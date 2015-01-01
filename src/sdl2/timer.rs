use std::mem;
use libc::{uint32_t, c_void};
use std::raw;
use std::kinds::marker::ContravariantLifetime;

pub use sys::timer as ll;

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
    lifetime: ContravariantLifetime<'a>,
}

impl<'a> Timer<'a> {
    pub fn new(delay: uint, callback: ||: 'a -> uint, remove_on_drop: bool) -> Timer<'a> {
        unsafe {
            let c_param = mem::transmute::<_, raw::Closure>(callback);
            Timer { delay: delay, raw: 0, closure: c_param, remove_on_drop: remove_on_drop, lifetime: ContravariantLifetime }
        }
    }

    pub fn start(&mut self) {
        unsafe {
            let timer_id = ll::SDL_AddTimer(
                    self.delay as u32, 
                    Some(c_timer_callback as
                        extern "C" fn (
                            _interval: uint32_t, 
                            param: *const c_void
                        ) -> uint32_t), 
                    mem::transmute(&self.closure)
                );
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
