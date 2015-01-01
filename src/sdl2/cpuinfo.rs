pub use sys::cpuinfo as ll;

pub const CACHELINESIZE: int = 128;

pub fn get_cpu_count() -> int {
    unsafe { ll::SDL_GetCPUCount() as int }
}

pub fn get_cpu_cache_line_size() -> int {
    unsafe { ll::SDL_GetCPUCacheLineSize() as int}
}

pub fn has_rdtsc() -> bool {
    unsafe { ll::SDL_HasRDTSC() == 1 }
}

pub fn has_alti_vec() -> bool {
    unsafe { ll::SDL_HasAltiVec() == 1 }
}

pub fn has_mmx() -> bool {
    unsafe { ll::SDL_HasMMX() == 1 }
}

pub fn has_3d_now() -> bool {
    unsafe { ll::SDL_Has3DNow() == 1 }
}

pub fn has_sse() -> bool {
    unsafe { ll::SDL_HasSSE() == 1 }
}

pub fn has_sse2() -> bool {
    unsafe { ll::SDL_HasSSE2() == 1 }
}

pub fn has_sse3() -> bool {
    unsafe { ll::SDL_HasSSE3() == 1 }
}

pub fn has_sse41() -> bool {
    unsafe { ll::SDL_HasSSE41() == 1 }
}

pub fn has_sse42() -> bool {
    unsafe { ll::SDL_HasSSE42() == 1 }
}

pub fn has_avx() -> bool {
    unsafe { ll::SDL_HasAVX() == 1 }
}

pub fn get_system_ram() -> int {
    unsafe { ll::SDL_GetSystemRAM() as int }
}
