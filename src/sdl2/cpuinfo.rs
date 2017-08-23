use sys;

pub const CACHELINESIZE: u8 = 128;

pub fn cpu_count() -> i32 {
    unsafe { sys::SDL_GetCPUCount() }
}

pub fn cpu_cache_line_size() -> i32 {
    unsafe { sys::SDL_GetCPUCacheLineSize() }
}

pub fn has_rdtsc() -> bool {
    unsafe { sys::SDL_HasRDTSC() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_alti_vec() -> bool {
    unsafe { sys::SDL_HasAltiVec() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_mmx() -> bool {
    unsafe { sys::SDL_HasMMX() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_3d_now() -> bool {
    unsafe { sys::SDL_Has3DNow() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_sse() -> bool {
    unsafe { sys::SDL_HasSSE() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_sse2() -> bool {
    unsafe { sys::SDL_HasSSE2() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_sse3() -> bool {
    unsafe { sys::SDL_HasSSE3() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_sse41() -> bool {
    unsafe { sys::SDL_HasSSE41() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_sse42() -> bool {
    unsafe { sys::SDL_HasSSE42() == sys::SDL_bool::SDL_TRUE }
}

pub fn has_avx() -> bool {
    unsafe { sys::SDL_HasAVX() == sys::SDL_bool::SDL_TRUE }
}

pub fn system_ram() -> i32 {
    unsafe { sys::SDL_GetSystemRAM() }
}
