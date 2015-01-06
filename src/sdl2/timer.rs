use std::mem;
use libc::{uint32_t, c_void};

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
    closure: &'a Box<FnMut() -> uint + 'a>,
}

impl<'a> Timer<'a> {
    pub fn new(delay: uint, callback: &'a Box<FnMut() -> uint + 'a>) -> Timer<'a> {
        Timer { delay: delay, raw: 0, closure: callback }
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
                    mem::transmute(self.closure)
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
        let ret = unsafe { ll::SDL_RemoveTimer(self.raw) };
        if ret != 1 {
            println!("error dropping timer {}, maybe already removed.", self.raw);
        }
    }
}

extern "C" fn c_timer_callback(_interval: uint32_t, param: *const c_void) -> uint32_t {
    let f: &mut Box<FnMut() -> uint> = unsafe { mem::transmute(param) };
    (*f)() as uint32_t
}

#[test]
fn test_timer_1() {
    use std::sync::{Arc, Mutex};

    let local_num: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let timer_num = local_num.clone();
    {
        let f: Box<FnMut() -> uint> = box |&mut:| {
            let mut num = timer_num.lock().unwrap();
            *num = *num + 1;
            10
        };
        let mut timer = Timer::new(10, &f);
        timer.start();
        delay(100);
        let num = local_num.lock().unwrap();
        assert!(*num == 9);
    }

    // Check that timer has stopped
    delay(100);
    let num = local_num.lock().unwrap();
    assert!(*num == 9);
}

#[test]
fn test_timer_2() {
    // Check that the closure lives long enough outside the block where
    // the timer was started.
    let f: Box<FnMut() -> uint> = box |&:| { 0 };
    let _ = {
        let mut timer = Timer::new(1000, &f);
        timer.start();
        timer
    };
    delay(200);
    delay(200);
    delay(200);
    delay(200);
    delay(200);
    delay(200);
}
