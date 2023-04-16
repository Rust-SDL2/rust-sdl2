use crate::sys;
use libc::c_void;
use std::marker::PhantomData;
use std::mem;

use crate::TimerSubsystem;

impl TimerSubsystem {
    /// Constructs a new timer using the boxed closure `callback`.
    ///
    /// The timer is started immediately, it will be cancelled either:
    ///
    /// * when the timer is dropped
    /// * or when the callback returns a non-positive continuation interval
    ///
    /// The callback is run in a thread that is created and managed internally
    /// by SDL2 from C. The callback *must* not panic!
    #[must_use = "if unused the Timer will be dropped immediately"]
    #[doc(alias = "SDL_AddTimer")]
    pub fn add_timer<'b, 'c>(&'b self, delay: u32, callback: TimerCallback<'c>) -> Timer<'b, 'c> {
        unsafe {
            let callback = Box::new(callback);
            let timer_id = sys::SDL_AddTimer(
                delay,
                Some(c_timer_callback),
                mem::transmute_copy(&callback),
            );

            Timer {
                callback: Some(callback),
                raw: timer_id,
                _marker: PhantomData,
            }
        }
    }

    /// Gets the number of milliseconds elapsed since the timer subsystem was initialized.
    ///
    /// It's recommended that you use another library for timekeeping, such as `time`.
    ///
    /// This function is not recommended in upstream SDL2 as of 2.0.18 and internally
    /// calls the 64-bit variant and masks the result.
    #[doc(alias = "SDL_GetTicks")]
    pub fn ticks(&self) -> u32 {
        // This is thread-safe as long as the ticks subsystem is inited, and
        // tying this to `TimerSubsystem` ensures the timer subsystem can
        // safely make calls into the ticks subsystem without invoking a
        // thread-unsafe `SDL_TicksInit()`.
        //
        // This binding is offered for completeness but is debatably a relic.
        unsafe { sys::SDL_GetTicks() }
    }

    /// Gets the number of milliseconds elapsed since the timer subsystem was initialized.
    ///
    /// It's recommended that you use another library for timekeeping, such as `time`.
    #[doc(alias = "SDL_GetTicks64")]
    pub fn ticks64(&self) -> u64 {
        // This is thread-safe as long as the ticks subsystem is inited, and
        // tying this to `TimerSubsystem` ensures the timer subsystem can
        // safely make calls into the ticks subsystem without invoking a
        // thread-unsafe `SDL_TicksInit()`.
        //
        // This binding is offered for completeness but is debatably a relic.
        unsafe { sys::SDL_GetTicks64() }
    }

    /// Sleeps the current thread for the specified amount of milliseconds.
    ///
    /// It's recommended that you use `std::thread::sleep()` instead.
    #[doc(alias = "SDL_Delay")]
    pub fn delay(&self, ms: u32) {
        // This is thread-safe as long as the ticks subsystem is inited, and
        // tying this to `TimerSubsystem` ensures the timer subsystem can
        // safely make calls into the ticks subsystem without invoking a
        // thread-unsafe `SDL_TicksInit()`.
        //
        // This binding is offered for completeness but is debatably a relic.
        unsafe { sys::SDL_Delay(ms) }
    }

    #[doc(alias = "SDL_GetPerformanceCounter")]
    pub fn performance_counter(&self) -> u64 {
        unsafe { sys::SDL_GetPerformanceCounter() }
    }

    #[doc(alias = "SDL_GetPerformanceFrequency")]
    pub fn performance_frequency(&self) -> u64 {
        unsafe { sys::SDL_GetPerformanceFrequency() }
    }
}

pub type TimerCallback<'a> = Box<dyn FnMut() -> u32 + 'a + Send>;

pub struct Timer<'b, 'a> {
    callback: Option<Box<TimerCallback<'a>>>,
    raw: sys::SDL_TimerID,
    _marker: PhantomData<&'b ()>,
}

impl<'b, 'a> Timer<'b, 'a> {
    /// Returns the closure as a trait-object and cancels the timer
    /// by consuming it...
    pub fn into_inner(mut self) -> TimerCallback<'a> {
        *self.callback.take().unwrap()
    }
}

impl<'b, 'a> Drop for Timer<'b, 'a> {
    #[inline]
    #[doc(alias = "SDL_RemoveTimer")]
    fn drop(&mut self) {
        // SDL_RemoveTimer returns SDL_FALSE if the timer wasn't found (impossible),
        // or the timer has been cancelled via the callback (possible).
        // The timer being cancelled isn't an issue, so we ignore the result.
        unsafe { sys::SDL_RemoveTimer(self.raw) };
    }
}

extern "C" fn c_timer_callback(_interval: u32, param: *mut c_void) -> u32 {
    // FIXME: This is UB if the callback panics! (But will realistically
    // crash on stack underflow.)
    //
    // I tried using `std::panic::catch_unwind()` here and it compiled but
    // would not catch. Maybe wait for `c_unwind` to stabilize? Then the behavior
    // will automatically abort the process when panicking over an `extern "C"`
    // function.
    let f = param as *mut TimerCallback<'_>;
    unsafe { (*f)() }
}

#[cfg(not(target_os = "macos"))]
#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_timer() {
        test_timer_runs_multiple_times();
        test_timer_runs_at_least_once();
        test_timer_can_be_recreated();
    }

    fn test_timer_runs_multiple_times() {
        let sdl_context = crate::sdl::init().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();

        let local_num = Arc::new(Mutex::new(0));
        let timer_num = local_num.clone();

        let _timer = timer_subsystem.add_timer(
            20,
            Box::new(|| {
                // increment up to 10 times (0 -> 9)
                // tick again in 100ms after each increment
                //
                let mut num = timer_num.lock().unwrap();
                if *num < 9 {
                    *num += 1;
                    20
                } else {
                    0
                }
            }),
        );

        // tick the timer at least 10 times w/ 200ms of "buffer"
        ::std::thread::sleep(Duration::from_millis(250));
        let num = local_num.lock().unwrap(); // read the number back
        assert_eq!(*num, 9); // it should have incremented at least 10 times...
    }

    fn test_timer_runs_at_least_once() {
        let sdl_context = crate::sdl::init().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();

        let local_flag = Arc::new(Mutex::new(false));
        let timer_flag = local_flag.clone();

        let _timer = timer_subsystem.add_timer(
            20,
            Box::new(|| {
                let mut flag = timer_flag.lock().unwrap();
                *flag = true;
                0
            }),
        );

        ::std::thread::sleep(Duration::from_millis(50));
        let flag = local_flag.lock().unwrap();
        assert_eq!(*flag, true);
    }

    fn test_timer_can_be_recreated() {
        let sdl_context = crate::sdl::init().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();

        let local_num = Arc::new(Mutex::new(0));
        let timer_num = local_num.clone();

        // run the timer once and reclaim its closure
        let timer_1 = timer_subsystem.add_timer(
            20,
            Box::new(move || {
                let mut num = timer_num.lock().unwrap();
                *num += 1; // increment the number
                0 // do not run timer again
            }),
        );

        // reclaim closure after timer runs
        ::std::thread::sleep(Duration::from_millis(50));
        let closure = timer_1.into_inner();

        // create a second timer and increment again
        let _timer_2 = timer_subsystem.add_timer(20, closure);
        ::std::thread::sleep(Duration::from_millis(50));

        // check that timer was incremented twice
        let num = local_num.lock().unwrap();
        assert_eq!(*num, 2);
    }
}
