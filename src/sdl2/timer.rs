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

pub type TimerCallback = extern "C" fn (interval: uint32_t, param: *const c_void) -> u32;

pub struct Timer {
    delay: uint,
    raw: ll::SDL_TimerID,
    callback: TimerCallback,
    param: *const c_void,
    remove_on_drop: bool,
}

impl Timer {
    pub fn new(delay: uint, callback: TimerCallback, param: *const c_void, remove_on_drop: bool) -> Timer {
        Timer { delay: delay, raw: 0, callback: callback, param: param, remove_on_drop: remove_on_drop }
    }

    pub fn start(&mut self) {
        unsafe {
            let timer_id = ll::SDL_AddTimer(self.delay as u32, Some(self.callback), self.param);
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
impl Drop for Timer {
    fn drop(&mut self) {
        if self.remove_on_drop {
            let ret = unsafe { ll::SDL_RemoveTimer(self.raw) };
            if ret != 1 {
                println!("error dropping timer {}, maybe already removed.", self.raw);
            }
        }
    }
}

#[cfg(test)]
extern "C" fn test_timer_1_callback(_interval: uint32_t, param: *const c_void) -> uint32_t {
    use std::sync::{Arc, Mutex};
    use std::mem;

    let locked_num: &Arc<Mutex<u32>> = unsafe { mem::transmute(param) };
    let mut num = locked_num.lock().unwrap();
    *num = *num + 1;
    10
}

#[test]
fn test_timer_1() {
    use std::sync::{Arc, Mutex};
    use std::mem;

    let local_num: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let timer_num = local_num.clone();
    {
        let param = unsafe { mem::transmute(&timer_num) };
        let mut timer = Timer::new(10, test_timer_1_callback, param, true);
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

#[cfg(test)]
extern "C" fn test_timer_2_callback(_interval: uint32_t, _param: *const c_void) -> uint32_t {
    0
}

#[test]
fn test_timer_2() {
    use std::ptr;

    let _ = {
        let mut timer = Timer::new(1000, test_timer_2_callback, ptr::null(), true);
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
