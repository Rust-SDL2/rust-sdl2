pub mod ll {

    use libc::{uint32_t, uint64_t};

    //SDL_timer.h
    extern "C" {
        pub fn SDL_GetTicks() -> uint32_t;
        pub fn SDL_GetPerformanceCounter() -> uint64_t;
        pub fn SDL_GetPerformanceFrequency() -> uint64_t;
        pub fn SDL_Delay(ms: uint32_t);
    }
    //TODO: Figure out what to do with the timer callback functions
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
