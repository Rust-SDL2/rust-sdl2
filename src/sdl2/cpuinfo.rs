#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int};

    pub type SDL_bool = c_int;

    // SDL_cpuinfo.h
    extern "C" {
        pub fn SDL_GetCPUCount() -> c_int;
        pub fn SDL_GetCPUCacheLineSize() -> c_int;
        pub fn SDL_HasRDTSC() -> SDL_bool;
        pub fn SDL_HasAltiVec() -> SDL_bool;
        pub fn SDL_HasMMX() -> SDL_bool;
        pub fn SDL_Has3DNow() -> SDL_bool;
        pub fn SDL_HasSSE() -> SDL_bool;
        pub fn SDL_HasSSE2() -> SDL_bool;
        pub fn SDL_HasSSE3() -> SDL_bool;
        pub fn SDL_HasSSE41() -> SDL_bool;
        pub fn SDL_HasSSE42() -> SDL_bool;
        pub fn SDL_HasAVX() -> SDL_bool;
        pub fn SDL_GetSystemRAM() -> c_int;
    }
}

pub static CachelineSize: int = 128;

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
