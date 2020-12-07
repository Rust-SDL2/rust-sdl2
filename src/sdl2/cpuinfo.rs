use crate::sys;
use crate::sys::SDL_bool;

pub const CACHELINESIZE: u8 = 128;

#[doc(alias = "SDL_GetCPUCount")]
pub fn cpu_count() -> i32 {
    unsafe { sys::SDL_GetCPUCount() }
}

#[doc(alias = "SDL_GetCPUCacheLineSize")]
pub fn cpu_cache_line_size() -> i32 {
    unsafe { sys::SDL_GetCPUCacheLineSize() }
}

#[doc(alias = "SDL_HasRDTSC")]
pub fn has_rdtsc() -> bool {
    unsafe { sys::SDL_HasRDTSC() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasAltiVec")]
pub fn has_alti_vec() -> bool {
    unsafe { sys::SDL_HasAltiVec() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasMMX")]
pub fn has_mmx() -> bool {
    unsafe { sys::SDL_HasMMX() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_Has3DNow")]
pub fn has_3d_now() -> bool {
    unsafe { sys::SDL_Has3DNow() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasSSE")]
pub fn has_sse() -> bool {
    unsafe { sys::SDL_HasSSE() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasSSE2")]
pub fn has_sse2() -> bool {
    unsafe { sys::SDL_HasSSE2() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasSSE3")]
pub fn has_sse3() -> bool {
    unsafe { sys::SDL_HasSSE3() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasSSE41")]
pub fn has_sse41() -> bool {
    unsafe { sys::SDL_HasSSE41() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasSSE42")]
pub fn has_sse42() -> bool {
    unsafe { sys::SDL_HasSSE42() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasAVX")]
pub fn has_avx() -> bool {
    unsafe { sys::SDL_HasAVX() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasAVX2")]
pub fn has_avx2() -> bool {
    unsafe { sys::SDL_HasAVX2() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_HasAVX512F")]
pub fn has_avx512f() -> bool {
    unsafe { sys::SDL_HasAVX512F() == SDL_bool::SDL_TRUE }
}

#[doc(alias = "SDL_GetSystemRAM")]
pub fn system_ram() -> i32 {
    unsafe { sys::SDL_GetSystemRAM() }
}
