use libc::{uint32_t, c_void};
use std::mem;
use sys::timer as ll;

pub fn get_ticks() -> u32 {
    unsafe { ll::SDL_GetTicks() }
}

pub fn get_performance_counter() -> u64 {
    unsafe { ll::SDL_GetPerformanceCounter() }
}

pub fn get_performance_frequency() -> u64 {
    unsafe { ll::SDL_GetPerformanceFrequency() }
}

pub fn delay(ms: u32) {
    unsafe { ll::SDL_Delay(ms) }
}

#[unstable = "Unstable because of move to unboxed closures and `box` syntax"]
pub struct Timer<F> {
    _callback: Box<F>,
    _delay: usize,
    raw: ll::SDL_TimerID,
}

impl<F> Timer<F> {
    /// Constructs a new timer using the boxed closure `callback`.
    /// The timer is started immediately, it will be cancelled either:
    ///   * when the timer is dropped
    ///   * or when the callback returns a non-positive continuation interval
    pub fn new(delay: usize, callback: Box<F>) -> Timer<F>
    where F: Fn() -> u32, F: Send {

        let timer = unsafe {
            let timer_id = ll::SDL_AddTimer(delay as u32,
                                            Some(c_timer_callback::<F>),
                                            mem::transmute_copy(&callback));

            Timer {
                _callback: callback,
                _delay: delay,
                raw: timer_id,
            }
        };

        return timer;
    }
}

#[unsafe_destructor]
impl<F> Drop for Timer<F> {
    fn drop(&mut self) {
        let ret = unsafe { ll::SDL_RemoveTimer(self.raw) };
        if ret != 1 {
            println!("error dropping timer {}, maybe already removed.", self.raw);
        }
    }
}

extern "C" fn c_timer_callback<F>(_interval: u32, param: *const c_void) -> uint32_t
where F: Fn() -> u32, F: Send {
    unsafe {
        let f: *const F = mem::transmute(param);
        (*f)() as uint32_t
    }
}

#[test]
fn test_timer_runs_multiple_times() {
    use std::sync::{Arc, Mutex};

    let local_num = Arc::new(Mutex::new(0));
    let timer_num = local_num.clone();

    let timer = Timer::new(100, Box::new(move|| {
        // increment up to 10 times (0 -> 9)
        // tick again in 100ms after each increment
        //
        let mut num = timer_num.lock().unwrap();
        if *num < 9 {
            *num += 1;
            100
        } else { 0 }
    }));

    delay(1200);                         // tick the timer at least 10 times w/ 200ms of "buffer"
    let num = local_num.lock().unwrap(); // read the number back
    assert_eq!(*num, 9);                 // it should have incremented at least 10 times...
}

#[test]
fn test_timer_runs_at_least_once() {
    use std::sync::{Arc, Mutex};

    let local_flag = Arc::new(Mutex::new(false));
    let timer_flag = local_flag.clone();

    let timer = Timer::new(500, Box::new(move|| {
        let mut flag = timer_flag.lock().unwrap();
        *flag = true; 0
    }));

    delay(700);
    let flag = local_flag.lock().unwrap();
    assert_eq!(*flag, true);
}
